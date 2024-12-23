use rhai::{CustomType, Engine, EvalAltResult, Scope, TypeBuilder};

pub fn do_thing() -> Result<(), Box<EvalAltResult>> {
    #[derive(Debug, Clone, CustomType)]
    #[rhai_type(extra = Self::build_extra)]
    struct TestStruct {
        x: i64,
    }

    impl TestStruct {
        pub fn new() -> Self {
            Self { x: 1 }
        }
        pub fn update(&mut self) {
            println!("x = {}", self.x);
            self.x += 1000;
        }
        pub fn calculate(&mut self, data: i64) -> i64 {
            self.x * data
        }
        fn build_extra(builder: &mut TypeBuilder<Self>) {
            builder
                .with_name("TestStruct")
                .with_fn("new_ts", Self::new)
                .with_fn("update", Self::update)
                .with_fn("calc", Self::calculate)
                .is_iterable();
        }
    }

    impl IntoIterator for TestStruct {
        type Item = i64;
        type IntoIter = std::vec::IntoIter<Self::Item>;

        #[inline]
        #[must_use]
        fn into_iter(self) -> Self::IntoIter {
            vec![self.x - 1, self.x, self.x + 1].into_iter()
        }
    }

    let mut engine = Engine::new();

    engine.build_type::<TestStruct>();

    let ast = engine.compile(
        "
        fn patch(app_id) {
            print(app_id);
        }
        ",
    )?;

    let mut scope = Scope::new();
    let result = engine.call_fn::<()>(&mut scope, &ast, "patch", (601,))?;

    Ok(())
}
