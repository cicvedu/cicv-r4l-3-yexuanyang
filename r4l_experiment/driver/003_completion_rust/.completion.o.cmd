cmd_/home/yyx/cicv-r4l-3-yexuanyang/r4l_experiment/driver/003_completion_rust/completion.o := RUST_MODFILE=/home/yyx/cicv-r4l-3-yexuanyang/r4l_experiment/driver/003_completion_rust/completion rustc --edition=2021 -Zbinary_dep_depinfo=y -Dunsafe_op_in_unsafe_fn -Drust_2018_idioms -Dunreachable_pub -Dnon_ascii_idents -Wmissing_docs -Drustdoc::missing_crate_level_docs -Dclippy::correctness -Dclippy::style -Dclippy::suspicious -Dclippy::complexity -Dclippy::perf -Dclippy::let_unit_value -Dclippy::mut_mut -Dclippy::needless_bitwise_bool -Dclippy::needless_continue -Wclippy::dbg_macro --target=./rust/target.json -Cpanic=abort -Cembed-bitcode=n -Clto=n -Cforce-unwind-tables=n -Ccodegen-units=1 -Csymbol-mangling-version=v0 -Crelocation-model=static -Zfunction-sections=n -Dclippy::float_arithmetic -Ctarget-feature=-sse,-sse2,-sse3,-ssse3,-sse4.1,-sse4.2,-avx,-avx2 -Ztune-cpu=generic -Cno-redzone=y -Ccode-model=kernel -Copt-level=2 -Cdebug-assertions=n -Coverflow-checks=y -Dwarnings  --cfg MODULE  @./include/generated/rustc_cfg -Zallow-features=allocator_api,bench_black_box,core_ffi_c,generic_associated_types,const_ptr_offset_from,const_refs_to_cell -Zcrate-attr=no_std -Zcrate-attr='feature(allocator_api,bench_black_box,core_ffi_c,generic_associated_types,const_ptr_offset_from,const_refs_to_cell)' --extern alloc --extern kernel --crate-type rlib --out-dir /home/yyx/cicv-r4l-3-yexuanyang/r4l_experiment/driver/003_completion_rust -L ./rust/ --crate-name completion --emit=dep-info,obj /home/yyx/cicv-r4l-3-yexuanyang/r4l_experiment/driver/003_completion_rust/completion.rs; mv /home/yyx/cicv-r4l-3-yexuanyang/r4l_experiment/driver/003_completion_rust/completion.d /home/yyx/cicv-r4l-3-yexuanyang/r4l_experiment/driver/003_completion_rust/.completion.o.d; sed -i '/^$(pound)/d' /home/yyx/cicv-r4l-3-yexuanyang/r4l_experiment/driver/003_completion_rust/.completion.o.d

source_/home/yyx/cicv-r4l-3-yexuanyang/r4l_experiment/driver/003_completion_rust/completion.o := /home/yyx/cicv-r4l-3-yexuanyang/r4l_experiment/driver/003_completion_rust/completion.rs

deps_/home/yyx/cicv-r4l-3-yexuanyang/r4l_experiment/driver/003_completion_rust/completion.o := \
  /home/yyx/cicv-r4l-3-yexuanyang/linux/rust/libcore.rmeta \
  /home/yyx/cicv-r4l-3-yexuanyang/linux/rust/libcompiler_builtins.rmeta \
  /home/yyx/cicv-r4l-3-yexuanyang/linux/rust/libkernel.rmeta \
  /home/yyx/cicv-r4l-3-yexuanyang/linux/rust/libbindings.rmeta \
  /home/yyx/cicv-r4l-3-yexuanyang/linux/rust/libmacros.so \
  /home/yyx/cicv-r4l-3-yexuanyang/linux/rust/liballoc.rmeta \
  /home/yyx/cicv-r4l-3-yexuanyang/linux/rust/libbuild_error.rmeta \
  /home/yyx/cicv-r4l-3-yexuanyang/linux/rust/libcore.rmeta \
  /home/yyx/cicv-r4l-3-yexuanyang/linux/rust/libcompiler_builtins.rmeta \
  /home/yyx/cicv-r4l-3-yexuanyang/linux/rust/libkernel.rmeta \
  /home/yyx/cicv-r4l-3-yexuanyang/linux/rust/libbindings.rmeta \
  /home/yyx/cicv-r4l-3-yexuanyang/linux/rust/libmacros.so \
  /home/yyx/cicv-r4l-3-yexuanyang/linux/rust/liballoc.rmeta \
  /home/yyx/cicv-r4l-3-yexuanyang/linux/rust/libbuild_error.rmeta \

/home/yyx/cicv-r4l-3-yexuanyang/r4l_experiment/driver/003_completion_rust/completion.o: $(deps_/home/yyx/cicv-r4l-3-yexuanyang/r4l_experiment/driver/003_completion_rust/completion.o)

$(deps_/home/yyx/cicv-r4l-3-yexuanyang/r4l_experiment/driver/003_completion_rust/completion.o):
