mod infra;

// Your tests go here!
success_tests! {
    {
        name: fact,
        file: "fact.snek",
        input: "10",
        expected: "3628800",
    },
    {
        name: even_odd_1,
        file: "even_odd.snek",
        input: "10",
        expected: "10\ntrue\ntrue",
    },
    {
        name: even_odd_2,
        file: "even_odd.snek",
        input: "9",
        expected: "9\nfalse\nfalse",
    },
    {
        name: lam0,
        file: "lam0.snek",
        input: "",
        expected: "6",
    },
    {
        name: lam1,
        file: "lam1.snek",
        input: "",
        expected: "50",
    },
    {
        name: lam2,
        file: "lam1.snek",
        input: "",
        expected: "(vec 6 50)",
    },
    {
        name: lam_fac,
        file: "lam-fac.snek",
        input: "5",
        expected: "120",
    },
    {
        name: lam_map,
        file: "lam-map.snek",
        input: "100",
        expected: "(vec 110 (vec 120 (vec 130 false)))",
    },


}

runtime_error_tests! {
    lam_arity: "arity mismatch",
    lam_not_fun: "callee is not a function",
}

static_error_tests! {
    {
        name: duplicate_params,
        file: "duplicate_params.snek",
        expected: "",
    }
}
