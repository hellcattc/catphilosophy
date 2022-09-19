import React from 'react'
import "./Post.css"
import MyImg from './MyImg'

const Post = (props) => {
  return (
    <div className = "post-container">
        <p style = {{textAlign: 'center'}}>{props.quote}</p>
        <MyImg src = {props.imgSrc}/>
    </div>
  )
}

export default Post