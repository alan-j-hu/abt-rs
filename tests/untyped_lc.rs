use abt::*;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Op {
    App,
    Lam,
}

const APP: &[Valence<()>] = &[Valence::new(&[], ()), Valence::new(&[], ())];
const LAM: &[Valence<()>] = &[Valence::new(&[()], ())];

impl Operator<()> for Op {
    fn arity<'a>(self: &Op) -> &'a [Valence<'a, ()>] {
        match self {
            Op::App => APP,
            Op::Lam => LAM,
        }
    }

    fn sort(self: &Op) -> () {
        ()
    }
}

#[test]
fn untyped_lc() {
    let mut supply = var::Supply::default();
    let var = supply.fresh(());
    let x = view::View::Var(var.clone()).to_abt().unwrap();
    let abs = view::AbsView(vec![var], x);

    let id_fun = view::View::Op(Op::Lam, vec![abs.clone()]).to_abt().unwrap();

    let _app_id = view::View::Op(Op::App, vec![id_fun.clone().into(), id_fun.clone().into()])
        .to_abt()
        .unwrap();
}

#[test]
fn malformed() {
    let mut supply = var::Supply::default();
    let var = supply.fresh(());
    let x = view::View::Var(var.clone()).to_abt().unwrap();
    let abs = view::AbsView(vec![], x);

    assert_eq!(view::View::Op(Op::Lam, vec![abs.clone()]).to_abt(), Err(()))
}
