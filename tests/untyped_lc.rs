use abt;

#[derive(Clone)]
enum Op {
    App,
    Lam,
}

struct Untyped;

const APP: abt::Arity<()> = abt::Arity::new(
    &[abt::Valence::new(&[], ()), abt::Valence::new(&[], ())],
    (),
);
const LAM: abt::Arity<()> = abt::Arity::new(&[abt::Valence::new(&[()], ())], ());

impl abt::Signature for Untyped {
    type Op = Op;
    type Sort = ();

    fn arity(op: &Op) -> &'static abt::Arity<()> {
        match op {
            Op::App => &APP,
            Op::Lam => &LAM,
        }
    }
}

#[test]
fn untyped_lc() {
    let mut supply = abt::var::Supply::default();
    let var = supply.fresh(());
    let x = abt::View::Var(var.clone()).to_abt::<Untyped>().unwrap();
    let abs = x.bind(&[var.clone()]);
    let id_fun = abt::View::Op(Op::Lam, vec![abs])
        .to_abt::<Untyped>()
        .unwrap();
    let _app_id = abt::View::Op(Op::App, vec![id_fun.bind(&[]), id_fun.bind(&[])])
        .to_abt::<Untyped>()
        .unwrap();
}
