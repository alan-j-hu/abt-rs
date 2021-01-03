use abt;

#[derive(Clone)]
enum Op {
    App,
    Lam,
}

struct Untyped;

impl abt::Signature for Untyped {
    type Op = Op;
    type Sort = ();

    fn arity(op: &Op) -> abt::Arity<()> {
        match *op {
            Op::App =>
                abt::Arity::new(
                    vec![
                        abt::Valence::new(vec![], ()),
                        abt::Valence::new(vec![], ())
                    ],
                    ()
                ),
            Op::Lam =>
                abt::Arity::new(
                    vec![abt::Valence::new(vec![()], ())],
                    ()
                ),
        }
    }
}

#[test]
fn untyped_lc() {
    let empty = &abt::context::Context::Empty;
    let id = abt::Abt::Op(Op::Lam, vec![abt::Abs(1, abt::Abt::Var(0))]);
    assert_eq!(id.has_sort::<Untyped>(&(), empty), true);
    let free = abt::Abt::Var(0);
    assert_eq!(free.has_sort::<Untyped>(&(), empty), false);
    let app_id = abt::Abt::Op(
        Op::App,
        vec![abt::Abs(0, id.clone()), abt::Abs(0, id.clone())]
    );
    assert_eq!(app_id.has_sort::<Untyped>(&(), empty), true)
}
