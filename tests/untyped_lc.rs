use abt;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Op {
    App,
    Lam,
}

const APP: &[abt::Valence<()>] = &[abt::Valence::new(&[], ()), abt::Valence::new(&[], ())];
const LAM: &[abt::Valence<()>] = &[abt::Valence::new(&[()], ())];

impl abt::Operator<()> for Op {
    fn arity<'a>(self: &Op) -> &'a [abt::Valence<()>] {
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
    let mut supply = abt::var::Supply::default();
    let var = supply.fresh(());
    let x = abt::View::Var(var.clone()).to_abt().unwrap();
    let abs = abt::AbsView(vec![var], x);

    let id_fun = abt::View::Op(Op::Lam, vec![abs.clone()]).to_abt().unwrap();

    let _app_id = abt::View::Op(Op::App, vec![id_fun.clone().into(), id_fun.clone().into()])
        .to_abt()
        .unwrap();
}

#[test]
fn malformed() {
    let mut supply = abt::var::Supply::default();
    let var = supply.fresh(());
    let x = abt::View::Var(var.clone()).to_abt().unwrap();
    let abs = abt::AbsView(vec![], x);

    assert_eq!(abt::View::Op(Op::Lam, vec![abs.clone()]).to_abt(), Err(()))
}
