Concept-Level Pre-Tokenization Compression for Large Language Models (LLMs)



Author: User-Originated Concept (formalized by Copilot)

&#x20;Working Name: Hierarchical Semantic Compression Layer (HSCL)

&#x20;Version: 1.0 Draft Whitepaper



Abstract



Modern Large Language Models tokenize text into subword units before processing. While highly effective, current tokenization schemes are fundamentally limited because they operate at the character, byte, word, or subword level.



This paper proposes a new preprocessing architecture called a Hierarchical Semantic Compression Layer (HSCL) that operates before tokenization.



Instead of tokenizing raw text directly, HSCL compresses common words, phrases, sentences, and eventually concepts into compact symbolic identifiers. The tokenizer then operates on these identifiers.



The central hypothesis is:



If semantically meaningful structures can be represented as compact symbols before tokenization, context capacity can increase dramatically without increasing model size.



This architecture could theoretically allow a model with a 128k token context window to process language equivalent to millions of words.



1\. Introduction



Current LLM pipeline:



Plain Text

Human Text

↓

Tokenizer

↓

Token IDs

↓

Embedding Layer

↓

Transformer

Show more lines



Example:



Plain Text

Hello world

Show more lines



Tokenizer:



Plain Text

\["Hello", " world"]

Show more lines



Token IDs:



Plain Text

\[15496, 995]

Show more lines



The model processes two tokens.



Even though "Hello world" is a common phrase, the model still allocates two token positions.



Problem Statement



Transformer cost grows approximately with sequence length.



If context doubles:



Plain Text

128k → 256k

Show more lines



memory and compute rise significantly.



Current approaches attack the problem by:



larger context windows

sparse attention

memory systems

retrieval systems



The proposed approach attacks the problem earlier:



Plain Text

Reduce sequence length before tokenization.

Show more lines

2\. Core Idea



Introduce an adaptive compression stage.



Instead of:



Plain Text

hello world

Show more lines



the model sees:



Plain Text

8 23

Show more lines



where:



Plain Text

8 = hello

23 = world

Show more lines



Pipeline:



Plain Text

Text

↓

Semantic Compression Layer

↓

Compressed Symbols

↓

Tokenizer

↓

Tokens

↓

Transformer

Show more lines

3\. Compression Levels



The system operates hierarchically.



Level 1: Word Compression



Dictionary:



Plain Text

1 = hello

2 = world

3 = computer

4 = tokenizer

Show more lines



Input:



Plain Text

hello computer world

Show more lines



Compressed:



Plain Text

1 3 2

Show more lines

Level 2: Phrase Compression



Dictionary:



Plain Text

101 = artificial intelligence

102 = machine learning

103 = neural network

Show more lines



Input:



Plain Text

artificial intelligence uses neural network methods

Show more lines



Compressed:



Plain Text

101 uses 103 methods

Show more lines

Level 3: Sentence Compression



Dictionary:



Plain Text

1001 = How are you today?

1002 = What is the weather?

1003 = I understand your question.

 

Show more lines



Input:



Plain Text

I understand your question.

Show more lines



Compressed:



Plain Text

1003

Show more lines



One symbol represents an entire sentence.



Level 4: Concept Compression



Dictionary:



Plain Text

50000 = concept of gravity

50001 = concept of democracy

50002 = concept of entropy

Show more lines



Input:



Plain Text

entropy increases in isolated systems

Show more lines



Compressed:



Plain Text

50002 increases in isolated systems

Show more lines

4\. Tokenization After Compression



Conventional tokenization:



Plain Text

artificial intelligence

 

Show more lines



might become:



Plain Text

\["artificial", " intelligence"]

``

Show more lines



Two tokens.



With HSCL:



Plain Text

101

Show more lines



Tokenizer:



Plain Text

\["101"]

Show more lines



One token.



Potential reduction:



Plain Text

2 → 1

Show more lines



For longer phrases:



Plain Text

20 → 1

Show more lines

5\. Dynamic Vocabulary Learning



The dictionary cannot be fixed.



It must evolve.



The system continuously learns:



Frequency



Often occurring patterns become symbols.



Example:



Plain Text

machine learning

Show more lines



appears millions of times.



Create:



Plain Text

ID 102

Show more lines

Compression Value



A phrase should only become a symbol if:



Plain Text

compression gained

>

storage cost

 

Show more lines

Semantic Stability



Good candidates:



Plain Text

United States

Artificial Intelligence

Machine Learning

New York City

Show more lines



Bad candidates:



Plain Text

random temporary word combinations

Show more lines

6\. Multi-Level Dictionary Design



Dictionary hierarchy:



Plain Text

Layer 1: Words

Layer 2: Phrases

Layer 3: Sentences

Layer 4: Concepts

Layer 5: Documents

Show more lines



Visualization:



Plain Text

Concept

├─ Phrase

│ ├─ Word

│ ├─ Word

│ └─ Word

└─ Phrase

Show more lines



This resembles a semantic tree.



7\. Training Procedure

Phase 1



Train compression layer.



Input:



Plain Text

massive corpus

Show more lines



Objective:



Plain Text

identify reusable structures

 

Show more lines

Phase 2



Generate compression dictionary.



Example:



Plain Text

1 = the

2 = and

501 = machine learning

4005 = artificial intelligence

70001 = climate change mitigation

Show more lines

Phase 3



Compress corpus.



Original:



Plain Text

The field of artificial intelligence is growing rapidly.

Show more lines



Compressed:



Plain Text

1 field of 4005 is growing rapidly.

Show more lines

Phase 4



Train transformer directly on compressed text.



No decompression occurs.



The model learns:



Plain Text

4005

Show more lines



means



Plain Text

artificial intelligence

Show more lines



intrinsically.



8\. Embedding Architecture



Traditional:



Plain Text

Word

↓

Embedding

Show more lines



HSCL:



Plain Text

Word

↓

Compressed Symbol

↓

Embedding

Show more lines



Example:



Plain Text

4005

Show more lines



gets its own embedding vector:



Plain Text

\[0.28, -1.55, 0.81, ...]

Show more lines



The model never expands it back.



The concept embedding exists directly.



9\. Potential Advantages

Massive Context Expansion



Example:



Current:



Plain Text

100,000 words

Show more lines



↓



Plain Text

130,000 tokens

Show more lines



After semantic compression:



Plain Text

20,000 tokens

``

Show more lines



Potential gain:



Plain Text

6× context density

Show more lines

Faster Inference



Shorter sequence:



Plain Text

less attention

less memory

less compute

Show more lines

Better Concept Retention



Instead of:



Plain Text

artificial

intelligence

Show more lines



the model sees:



Plain Text

AI\_CONCEPT

Show more lines



as a single unit.



Reduced Redundancy



Repeated structures become:



Plain Text

one symbol reused millions of times

Show more lines

10\. Challenges

Loss of Granularity



Current model:



Plain Text

artificial

intelligence

Show more lines



Model can reason about components.



Compressed:



Plain Text

4005

Show more lines



Internal structure hidden.



Potential reasoning loss.



Dictionary Explosion



Millions of concepts:



Plain Text

millions of IDs

Show more lines



Need efficient management.



Ambiguity



Example:



Plain Text

Apple

Show more lines



Could mean:



Plain Text

fruit

company

Show more lines



Requires context-sensitive mapping.



Generalization



Current tokenizers handle new words easily through subwords.



Concept compression may struggle with new concepts.



11\. Hybrid Solution



Most practical architecture:



Plain Text

Raw Text

↓

Concept Compressor

↓

Concept IDs

\+

Subwords

↓

Transformer

Show more lines



Example:



Plain Text

Artificial intelligence will transform society.

Show more lines



becomes:



Plain Text

4005 will transform society.

Show more lines



where only known concepts are compressed.



Unknown text remains normal.



12\. Mathematical Formulation



Let:



Plain Text

L = original token count

Show more lines



Let:



Plain Text

C = compressed token count

Show more lines



Compression ratio:



Plain Text

R = L / C

Show more lines



Example:



Plain Text

100000 / 20000

Show more lines

Plain Text

R = 5

Show more lines



Meaning:



Plain Text

5× more information per token

Show more lines

13\. Long-Term Evolution



Future versions might compress:



Concepts

Plain Text

democracy

gravity

economics

Show more lines



into single symbols.



Arguments



Entire reasoning chains become symbols.



Knowledge Structures



A scientific paper may compress into:



Plain Text

DOC\_81924

Show more lines



with the model internally understanding its structure.



14\. Conclusion



The Hierarchical Semantic Compression Layer proposes moving compression upstream from tokenization. Instead of representing language as characters, bytes, or subwords, the architecture represents increasingly larger semantic structures as compact symbolic identifiers prior to tokenization.



The key innovation is not merely text compression but semantic compression: words, phrases, concepts, and recurring ideas become atomic units of representation. If successful, this approach could significantly expand effective context windows, reduce computational requirements, and enable transformers to operate on concept-level abstractions rather than surface-level language.



The primary research challenge is balancing compression efficiency against the loss of compositional reasoning. A hybrid architecture combining semantic compression with conventional subword tokenization is likely the most practical path toward implementation.

