use abt;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Op {
    App,
    Lam,
}

struct Untyped;

const APP: &[abt::Valence<()>] = &[abt::Valence::new(&[], ()), abt::Valence::new(&[], ())];
const LAM: &[abt::Valence<()>] = &[abt::Valence::new(&[()], ())];

impl abt::Signature for Untyped {
    type Op = Op;
    type Sort = ();

    fn arity<'a>(op: &Op) -> &'a [abt::Valence<()>] {
        match op {
            Op::App => APP,
            Op::Lam => LAM,
        }
    }

    fn sort(_: &Op) -> () {
        ()
    }
}

#[test]
fn untyped_lc() {
    let mut supply = abt::var::Supply::default();
    let var = supply.fresh(());
    let x = abt::View::Var(var.clone()).to_abt::<Untyped>().unwrap();
    let abs = x.bind(&[var.clone()]);

    let id_fun = abt::View::Op(Op::Lam, vec![abs.clone()])
        .to_abt::<Untyped>()
        .unwrap();
    assert_eq!(id_fun.view(), abt::View::Op(Op::Lam, vec![abs.clone()]));

    let app_id = abt::View::Op(Op::App, vec![id_fun.bind(&[]), id_fun.bind(&[])])
        .to_abt::<Untyped>()
        .unwrap();
    assert_eq!(
        app_id.view(),
        abt::View::Op(Op::App, vec![id_fun.bind(&[]), id_fun.bind(&[])])
    )
}

#[test]
fn malformed() {
    let mut supply = abt::var::Supply::default();
    let var = supply.fresh(());
    let x = abt::View::Var(var.clone()).to_abt::<Untyped>().unwrap();
    let abs = x.bind(&[]);

    assert_eq!(
        abt::View::Op(Op::Lam, vec![abs.clone()]).to_abt::<Untyped>(),
        Err(())
    )
}
