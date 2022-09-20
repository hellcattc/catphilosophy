import {
  useEffect, useState, useCallback, useRef, React,
} from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import './App.css';
import Post from './Components/Post/Post';

function App() {
  const [contentFeed, setContentFeed] = useState([]);
  const [postsPage, setPostsPage] = useState(0);
  const [loadingPosts, setLoadingPosts] = useState(false);
  const loader = useRef(0);

  const handleObserver = useCallback((entries) => {
    const target = entries[0];
    if (target.isIntersecting) {
      setPostsPage((prev) => prev + 1);
    }
  }, []);

  useEffect(() => {
    const option = {
      root: null,
      rootMargin: '20px',
      threshold: 0,
    };
    const observer = new IntersectionObserver(handleObserver, option);
    if (loader.current) observer.observe(loader.current);
  }, [handleObserver]);

  async function getData() {
    if (!loadingPosts) {
      setLoadingPosts(true);
      await invoke('get_text_and_photos', { postCount: 3 })
        .then((res) => {
          const postArray = res;
          console.log(res);
          setContentFeed([...contentFeed, ...postArray]);
          setLoadingPosts(false);
        })
        .catch((e) => console.log(e));
    }
  }

  useEffect(() => {
    getData();
  }, []);

  useEffect(() => {
    getData();
  }, [postsPage]);

  const contentFeedPosts = contentFeed.map((obj, i) => (
    <Post
    // eslint-disable-next-line react/no-array-index-key
      key={i}
      imgSrc={obj.imgUrl}
      quote={obj.quoteData.quote}
      author={obj.quoteData.author ? obj.quoteData.author : undefined}
    />
  ));

  return (
    <div>
      {contentFeedPosts}
      <div ref={loader} />
    </div>
  );
}

export default App;
