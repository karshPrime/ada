session_root "~/Projects/ada"

if initialize_session "ada"; then
    new_window "code"
    run_cmd "nvim src/main.rs"

    new_window "shell"
    split_h
    run_cmd "ssh core3b+"
    split_v
    run_cmd "cd ~"

    select_window 1
fi

finalize_and_go_to_session
