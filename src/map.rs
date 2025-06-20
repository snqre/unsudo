

#[derive(Clone)]
#[derive(Default)]
#[repr(transparent)]
pub struct Strings<const T: usize>(heapless::String<T>);

impl<const T: usize> String<T> {

    /// Returns an iterator over the [`char`]s of a string slice.
    ///
    /// As a string slice consists of valid UTF-8, we can iterate through a
    /// string slice by [`char`]. This method returns such an iterator.
    ///
    /// It's important to remember that [`char`] represents a Unicode Scalar
    /// Value, and might not match your idea of what a 'character' is. Iteration
    /// over grapheme clusters may be what you actually want. This functionality
    /// is not provided by Rust's standard library, check crates.io instead.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let word = "goodbye";
    ///
    /// let count = word.chars().count();
    /// assert_eq!(7, count);
    ///
    /// let mut chars = word.chars();
    ///
    /// assert_eq!(Some('g'), chars.next());
    /// assert_eq!(Some('o'), chars.next());
    /// assert_eq!(Some('o'), chars.next());
    /// assert_eq!(Some('d'), chars.next());
    /// assert_eq!(Some('b'), chars.next());
    /// assert_eq!(Some('y'), chars.next());
    /// assert_eq!(Some('e'), chars.next());
    ///
    /// assert_eq!(None, chars.next());
    /// ```
    ///
    /// Remember, [`char`]s might not match your intuition about characters:
    ///
    /// [`char`]: prim@char
    ///
    /// ```
    /// let y = "y̆";
    ///
    /// let mut chars = y.chars();
    ///
    /// assert_eq!(Some('y'), chars.next()); // not 'y̆'
    /// assert_eq!(Some('\u{0306}'), chars.next());
    ///
    /// assert_eq!(None, chars.next());
    /// ```
    pub fn chars(&self) -> str::Chars<'_> {
        self.0.chars()
    }

    /// Returns an iterator over the [`char`]s of a string slice, and their
    /// positions.
    ///
    /// As a string slice consists of valid UTF-8, we can iterate through a
    /// string slice by [`char`]. This method returns an iterator of both
    /// these [`char`]s, as well as their byte positions.
    ///
    /// The iterator yields tuples. The position is first, the [`char`] is
    /// second.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let word = "goodbye";
    ///
    /// let count = word.char_indices().count();
    /// assert_eq!(7, count);
    ///
    /// let mut char_indices = word.char_indices();
    ///
    /// assert_eq!(Some((0, 'g')), char_indices.next());
    /// assert_eq!(Some((1, 'o')), char_indices.next());
    /// assert_eq!(Some((2, 'o')), char_indices.next());
    /// assert_eq!(Some((3, 'd')), char_indices.next());
    /// assert_eq!(Some((4, 'b')), char_indices.next());
    /// assert_eq!(Some((5, 'y')), char_indices.next());
    /// assert_eq!(Some((6, 'e')), char_indices.next());
    ///
    /// assert_eq!(None, char_indices.next());
    /// ```
    ///
    /// Remember, [`char`]s might not match your intuition about characters:
    ///
    /// [`char`]: prim@char
    ///
    /// ```
    /// let yes = "y̆es";
    ///
    /// let mut char_indices = yes.char_indices();
    ///
    /// assert_eq!(Some((0, 'y')), char_indices.next()); // not (0, 'y̆')
    /// assert_eq!(Some((1, '\u{0306}')), char_indices.next());
    ///
    /// // note the 3 here - the previous character took up two bytes
    /// assert_eq!(Some((3, 'e')), char_indices.next());
    /// assert_eq!(Some((4, 's')), char_indices.next());
    ///
    /// assert_eq!(None, char_indices.next());
    /// ```
    #[inline]
    pub fn char_indices(&self) -> str::CharIndices<'_> {
        self.0.char_indices()
    }
}

impl<const T: usize> String<T> {
    /// Returns the maximum number of elements the String can hold
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use heapless::String;
    ///
    /// let mut s: String<4> = String::new();
    /// assert!(s.capacity() == 4);
    /// ```
    pub const fn cap(&self) -> usize {
        T
    }

    /// Returns the length of `self`.
    ///
    /// This length is in bytes, not [`char`]s or graphemes. In other words,
    /// it might not be what a human considers the length of the string.
    ///
    /// [`char`]: prim@char
    ///
    /// # Examples
    ///
    /// ```
    /// let len = "foo".len();
    /// assert_eq!(3, len);
    ///
    /// assert_eq!("ƒoo".len(), 4); // fancy f!
    /// assert_eq!("ƒoo".chars().count(), 3);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if `self` has a length of zero bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// let s = "";
    /// assert!(s.is_empty());
    ///
    /// let s = "not empty";
    /// assert!(!s.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<const T: usize> String<T> {

    /// Appends a given string slice onto the end of this `String`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use heapless::String;
    ///
    /// let mut s: String<8> = String::try_from("foo")?;
    ///
    /// assert!(s.push_str("bar").is_ok());
    ///
    /// assert_eq!("foobar", s);
    ///
    /// assert!(s.push_str("tender").is_err());
    /// # Ok::<(), ()>(())
    /// ```
    #[inline]
    pub fn push_str(&mut self, s: &str) -> Result<()> {
        self.0.push_str(s).map_err(|_| Error::Overflow)?;
        Ok(())
    }

    /// Appends the given [`char`] to the end of this `String`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use heapless::String;
    ///
    /// let mut s: String<8> = String::try_from("abc")?;
    ///
    /// s.push('1').unwrap();
    /// s.push('2').unwrap();
    /// s.push('3').unwrap();
    ///
    /// assert!("abc123" == s.as_str());
    ///
    /// assert_eq!("abc123", s);
    /// # Ok::<(), ()>(())
    /// ```
    #[inline]
    pub fn push(&mut self, c: char) -> Result<()> {
        self.0.push(c).map_err(|_| Error::Overflow)?;
        Ok(())
    }
}

impl<const T: usize> String<T> {
    pub fn pop(&mut self) -> Result<()> {
        self.0.pop().ok_or(Error::Empty)?;
        Ok(())
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }









    pub fn truncate(&mut self, new_len: usize) {
        self.0.truncate(new_len);
    }
}

impl<const T: usize> String<T> {

    /// Returns `true` if the given pattern matches a sub-slice of
    /// this string slice.
    ///
    /// Returns `false` if it does not.
    ///
    /// The [pattern] can be a `&str`, [`char`], a slice of [`char`]s, or a
    /// function or closure that determines if a character matches.
    ///
    /// [`char`]: prim@char
    /// [pattern]: self::pattern
    ///
    /// # Examples
    ///
    /// ```
    /// let bananas = "bananas";
    ///
    /// assert!(bananas.contains("nana"));
    /// assert!(!bananas.contains("apples"));
    /// ```
    #[inline]
    pub fn contains(&self, sample: &str) -> bool {
        self.0.contains(sample)
    }
    
    /// Returns `true` if the given pattern matches a prefix of this
    /// string slice.
    ///
    /// Returns `false` if it does not.
    ///
    /// The [pattern] can be a `&str`, in which case this function will return true if
    /// the `&str` is a prefix of this string slice.
    ///
    /// The [pattern] can also be a [`char`], a slice of [`char`]s, or a
    /// function or closure that determines if a character matches.
    /// These will only be checked against the first character of this string slice.
    /// Look at the second example below regarding behavior for slices of [`char`]s.
    ///
    /// [`char`]: prim@char
    /// [pattern]: self::pattern
    ///
    /// # Examples
    ///
    /// ```
    /// let bananas = "bananas";
    ///
    /// assert!(bananas.starts_with("bana"));
    /// assert!(!bananas.starts_with("nana"));
    /// ```
    ///
    /// ```
    /// let bananas = "bananas";
    ///
    /// // Note that both of these assert successfully.
    /// assert!(bananas.starts_with(&['b', 'a', 'n', 'a']));
    /// assert!(bananas.starts_with(&['a', 'b', 'c', 'd']));
    /// ```
    #[inline]
    pub fn starts_with(&self, sample: &str) -> bool {
        self.0.starts_with(sample)
    }

    /// Returns `true` if the given pattern matches a suffix of this
    /// string slice.
    ///
    /// Returns `false` if it does not.
    ///
    /// The [pattern] can be a `&str`, [`char`], a slice of [`char`]s, or a
    /// function or closure that determines if a character matches.
    ///
    /// [`char`]: prim@char
    /// [pattern]: self::pattern
    ///
    /// # Examples
    ///
    /// ```
    /// let bananas = "bananas";
    ///
    /// assert!(bananas.ends_with("anas"));
    /// assert!(!bananas.ends_with("nana"));
    /// ```
    #[inline]
    pub fn ends_with(&self, sample: &str) -> bool {
        self.0.ends_with(sample)
    }
}

impl<const T: usize> String<T> {

    /// Returns a string slice with leading and trailing whitespace removed.
    ///
    /// 'Whitespace' is defined according to the terms of the Unicode Derived
    /// Core Property `White_Space`, which includes newlines.
    ///
    /// # Examples
    ///
    /// ```
    /// let s = "\n Hello\tworld\t\n";
    ///
    /// assert_eq!("Hello\tworld", s.trim());
    /// ```
    #[inline]
    pub fn trim(&self) -> &str {
        self.0.trim()
    }

    /// Returns a string slice with leading and trailing ASCII whitespace
    /// removed.
    ///
    /// 'Whitespace' refers to the definition used by
    /// [`u8::is_ascii_whitespace`].
    ///
    /// [`u8::is_ascii_whitespace`]: u8::is_ascii_whitespace
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!("\r hello world\n ".trim_ascii(), "hello world");
    /// assert_eq!("  ".trim_ascii(), "");
    /// assert_eq!("".trim_ascii(), "");
    /// ``
    #[inline]
    pub fn trim_ascii(&self) -> &str {
        self.0.trim_ascii()
    }

    /// Returns a string slice with trailing ASCII whitespace removed.
    ///
    /// 'Whitespace' refers to the definition used by
    /// [`u8::is_ascii_whitespace`].
    ///
    /// [`u8::is_ascii_whitespace`]: u8::is_ascii_whitespace
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!("\r hello world\u{3000}\n ".trim_ascii_end(), "\r hello world\u{3000}");
    /// assert_eq!("  ".trim_ascii_end(), "");
    /// assert_eq!("".trim_ascii_end(), "");
    /// ```
    #[inline]
    pub fn trim_ascii_end(&self) -> &str {
        self.0.trim_ascii_end()
    }

    /// Returns a string slice with leading ASCII whitespace removed.
    ///
    /// 'Whitespace' refers to the definition used by
    /// [`u8::is_ascii_whitespace`].
    ///
    /// [`u8::is_ascii_whitespace`]: u8::is_ascii_whitespace
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!(" \t \u{3000}hello world\n".trim_ascii_start(), "\u{3000}hello world\n");
    /// assert_eq!("  ".trim_ascii_start(), "");
    /// assert_eq!("".trim_ascii_start(), "");
    /// ```
    #[inline]
    pub fn trim_ascii_start(&self) -> &str {
        self.0.trim_ascii_start()
    }

    /// Returns a string slice with trailing whitespace removed.
    ///
    /// 'Whitespace' is defined according to the terms of the Unicode Derived
    /// Core Property `White_Space`, which includes newlines.
    ///
    /// # Text directionality
    ///
    /// A string is a sequence of bytes. `end` in this context means the last
    /// position of that byte string; for a left-to-right language like English or
    /// Russian, this will be right side, and for right-to-left languages like
    /// Arabic or Hebrew, this will be the left side.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let s = "\n Hello\tworld\t\n";
    /// assert_eq!("\n Hello\tworld", s.trim_end());
    /// ```
    ///
    /// Directionality:
    ///
    /// ```
    /// let s = "  English  ";
    /// assert!(Some('h') == s.trim_end().chars().rev().next());
    ///
    /// let s = "  עברית  ";
    /// assert!(Some('ת') == s.trim_end().chars().rev().next());
    /// ```
    #[inline]
    pub fn trim_end(&self) -> &str {
        self.0.trim_end()
    }

    /// Returns a string slice with leading whitespace removed.
    ///
    /// 'Whitespace' is defined according to the terms of the Unicode Derived
    /// Core Property `White_Space`, which includes newlines.
    ///
    /// # Text directionality
    ///
    /// A string is a sequence of bytes. `start` in this context means the first
    /// position of that byte string; for a left-to-right language like English or
    /// Russian, this will be left side, and for right-to-left languages like
    /// Arabic or Hebrew, this will be the right side.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let s = "\n Hello\tworld\t\n";
    /// assert_eq!("Hello\tworld\t\n", s.trim_start());
    /// ```
    ///
    /// Directionality:
    ///
    /// ```
    /// let s = "  English  ";
    /// assert!(Some('E') == s.trim_start().chars().next());
    ///
    /// let s = "  עברית  ";
    /// assert!(Some('ע') == s.trim_start().chars().next());
    /// ```
    #[inline]
    pub fn trim_start(&self) -> &str {
        self.0.trim_start()
    }
}

impl<const T: usize> String<T> {
    /// Converts a string slice to a byte slice. To convert the byte slice back
    /// into a string slice, use the [`from_utf8`] function.
    ///
    /// # Examples
    ///
    /// ```
    /// let bytes = "bors".as_bytes();
    /// assert_eq!(b"bors", bytes);
    /// ```
    #[inline]
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    /// Extracts a string slice containing the entire string.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use heapless::String;
    ///
    /// let mut s: String<4> = String::try_from("ab")?;
    /// assert!(s.as_str() == "ab");
    ///
    /// let _s = s.as_str();
    /// // s.push('c'); // <- cannot borrow `s` as mutable because it is also borrowed as immutable
    /// # Ok::<(), ()>(())
    /// ```
    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl<const T: usize> ops::Add for String<T> {
    type Output = Result<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut out = Self::default();
        let x = self.as_str();
        let y = rhs.as_str();
        out.push_str(x)?;
        out.push_str(y)?;
        Ok(out)
    }
}