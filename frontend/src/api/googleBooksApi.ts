/**
 * Fetches book details from Google Books API.
 *
 * @param bookId - The ID of the book to fetch.
 */
export const fetchBookDetails = async (bookId: string) => {
  try {
    const response = await fetch(`https://www.googleapis.com/books/v1/volumes/${bookId}`);
    if (response.ok) {
      return await response.json();
    } else {
      console.error('Failed to fetch book details:', await response.json());
      return null;
    }
  } catch (error) {
    console.error('Failed to fetch book details:', error);
    return null;
  }
};

/**
 * Fetches books from Google Books API.
 *
 * @param query - The search query.
 */
export const searchBooks = async (query: string) => {
  try {
    const response = await fetch(`https://www.googleapis.com/books/v1/volumes?q=${encodeURIComponent(query)}`);
    if (response.ok) {
      const data = await response.json();
      return data.items || [];
    } else {
      console.error('Failed to fetch books:', await response.json());
      return [];
    }
  } catch (error) {
    console.error('Failed to fetch books:', error);
    return [];
  }
};
