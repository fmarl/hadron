(use-modules (guix gexp)
             (guix packages)
             (guix download)
             (guix profiles)
             (guix search-paths)
             (gnu packages llvm)
             (gnu packages rust))

(define rust-library-lock
  (origin
    (method url-fetch)
    (uri (string-append "https://raw.githubusercontent.com/rust-lang/rust/"
                        (package-version rust) "/library/Cargo.lock"))
    (sha256
     (base32 "0yd23c5xqjmjzrrqjiq7c33qn9a067sncwwb1hc49nijwg24y1y2"))))

(define rust-library-src
  (computed-file
   "rust-library-src"
   (with-imported-modules '((guix build utils))
     #~(begin
         (use-modules (guix build utils))
         (let ((library (string-append #$output "/lib/rustlib/src/rust/library")))
           (copy-recursively
            (string-append #$rust:rust-src "/lib/rustlib/src/rust/library")
            library)
           (make-file-writable (string-append library "/Cargo.lock"))
           (copy-file #$rust-library-lock
                      (string-append library "/Cargo.lock")))))))

(define rust-lld
  (computed-file
   "rust-lld"
   #~(begin
       (mkdir #$output)
       (mkdir (string-append #$output "/bin"))
       (symlink (string-append #$lld "/bin/lld")
                (string-append #$output "/bin/rust-lld")))))

(concatenate-manifests
 (list
  (manifest
   (list (manifest-entry
           (name "rust-library-src")
           (version (package-version rust))
           (item rust-library-src)
           (search-paths
            (list (search-path-specification
                   (variable "__CARGO_TESTS_ONLY_SRC_ROOT")
                   (files '("lib/rustlib/src/rust/library"))
                   (file-type 'directory)
                   (separator #f)))))
         (manifest-entry
           (name "rust-lld")
           (version (package-version lld))
           (item rust-lld))))
  (specifications->manifest
   '("rust"
     "rust:cargo"
     "rust:rust-src"
     "rust:tools"
     "make"
     "gcc-toolchain"
     "coreutils"
     "git"
     "nss-certs"
     "nasm"
     "mtools"
     "util-linux"
     "qemu"
     "gdb"))))
