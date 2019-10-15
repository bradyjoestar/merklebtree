<h1 align="center">Merkle BTree in Rust</h1>
<p >
<a href="#"><img src="https://img.shields.io/badge/version-0.1.0-brightgreen.svg" alt="Version"></a>
</p>

A content addressed B-tree backed by a content addressed hashtable.

Each tree node is stored as an object in the content addressed storage, and contains links to its children. Each link is a hash which can be loooked up from the content addressed storage.

### License
The code is available under the Apache license.