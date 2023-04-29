# leia-rust

LEIA stands for Learning Expectations Incrementally Autonomously.

I still haven't found out what _Learning_ means.

Many Machine Learning systems aim to _predict_ the behavior of the environment. In contrast, LEIA aims to adapt and adjust its _expectations_.

Many Machine Learning systems are designed to do their learning on a training set and be subsequently evaluated on a set of test data. After that the model can be used on unclassified data. In contrast, LEIA starts with the assumption that each possible future is equally likely, and starts learning immediately. It learns _incrementally_.

Many Learning Systems are designed to be taught. An external conscious experimenter has to prepare the training set and the test set and label each example according to the meaning that the experimenter gives to the data. In contrast, LEIA is designed to learn _autonomously_, in the sense that there is no predefined meaning in the data. LEIA 'just' learns what to expect. The meaning of the signals emerges from the interactions.

This project is an implementation of LEIA-learns in Rust.

Members (sub-projects):

* compute-oracle: Computes a list of pairs. Each pair consists a fraction between 0 and 1 and an  integer rank. This list is used bij LEIA learns to split measurements in two parts: a model and a description of the measurements given that model.
* leia-listens: tracks a potential infinite sequence of signals. The signals are only labeled _inbound_ or _outbound_. No further meaning is pre-given. After each prefix of the sequence of symbols, LEIA has a model that can be used to compute the likelihood of any continuation of the sequence.
* leia-talks: Decides which (if any) outbound signal to generate after each prefix of the sequence.