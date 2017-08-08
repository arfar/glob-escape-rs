glob-escape-rs
##############

This is a learning project for me to learn how to write Rust. The problem I'm
solving here is "real life" in that I needed to escape some key characters in a
glob string. I'm going to loosely base this on my `previous C version
<https://github.com/arfar/glob-escape>`_ (which is
pretty amatuer, I know).

glob-escape is a small library because I quite stupidly didn't realise that the
``[`` and ``]`` characters in my folders I was trying to use glob in were also
being consumed by glob itself.

TODO
====

* Improve implementation

  - I think I'll need some help here, I'm unsure:

    #. what a more efficient algorithm would be

    #. how to do partial string copy in rust


* Write docs

* Improve tests to call glob and show actual use cases

* Use library (relies on other project in C calling this rust library)

License
=======

LGPLv3
