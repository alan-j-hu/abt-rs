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

fn cbn(term: &Abt<Op, ()>) -> Abt<Op, ()> {
    let mut supply = var::Supply::new();
    match term.view(&mut supply) {
        view::View::Var(var) => view::View::Var(var.clone()).to_abt().unwrap(),
        view::View::Op(Op::App, rands) => match rands[0].1.view(&mut supply) {
            view::View::Var(var) => view::View::Op(
                Op::App,
                vec![
                    view::View::Var(var.clone()).to_abt().unwrap().into(),
                    rands[1].clone(),
                ],
            )
            .to_abt()
            .unwrap(),
            view::View::Op(Op::App, _) => {
                view::View::Op(Op::App, vec![cbn(&rands[0].1).into(), rands[1].clone()])
                    .to_abt()
                    .unwrap()
            }
            view::View::Op(Op::Lam, body) => body[0].1.subst(&body[0].0[0], &rands[1].1),
        },
        view::View::Op(Op::Lam, _) => term.clone(),
    }
}

#[test]
fn untyped_lc() {
    let mut supply = var::Supply::default();
    let var = supply.fresh(());
    let x = view::View::Var(var.clone()).to_abt().unwrap();
    let abs = view::AbsView(vec![var], x);

    let id_fun = view::View::Op(Op::Lam, vec![abs.clone()]).to_abt().unwrap();

    let app_id = view::View::Op(Op::App, vec![id_fun.clone().into(), id_fun.clone().into()])
        .to_abt()
        .unwrap();
    assert_eq!(id_fun, id_fun.view(&mut supply).to_abt().unwrap());
    assert_eq!(app_id, app_id.view(&mut supply).to_abt().unwrap());
    assert_eq!(id_fun, cbn(&app_id))
}

#[test]
fn malformed() {
    let mut supply = var::Supply::default();
    let var = supply.fresh(());
    let x = view::View::Var(var.clone()).to_abt().unwrap();
    let abs = view::AbsView(vec![], x);

    assert_eq!(view::View::Op(Op::Lam, vec![abs.clone()]).to_abt(), Err(()))
}
