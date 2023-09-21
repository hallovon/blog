<script>
    let visible = true;

    const handleOnScroll = () => {
        let scrollTop = Math.max(
            document.body.scrollTop,
            document.documentElement.scrollTop
        );
        window.onscroll = () => {
            if (scrollTop >= window.screen.height) {
                visible = false;
            } else {
                visible = true;
            }
        };
    };
</script>

<svelte:window on:scroll={handleOnScroll} />

{#if visible}
    <header id="header">
        <h3 class="name"><a href="/" class="item">hallovon</a></h3>
        <ul class="list">
            <li>
                <a href="/" class="item">
                    <ion-icon name="home" />
                    <span>首页</span>
                </a>
            </li>
            <li>
                <a href="/" class="item">
                    <ion-icon name="archive" />
                    <span>归档</span>
                </a>
            </li>
            <li>
                <a href="/" class="item">
                    <ion-icon name="bookmark" />
                    <span>标签</span>
                </a>
            </li>
            <li>
                <a href="/" class="item">
                    <ion-icon name="id-card" />
                    <span>关于</span>
                </a>
            </li>
            <li>
                <a href="/" class="item">
                    <ion-icon name="document-text" />
                    <span>留言</span>
                </a>
            </li>
        </ul>
    </header>
{/if}

<style scoped>
    #header {
        font-size: 1.8rem;
        width: 100%;
        height: 7.2rem;
        padding: 0 4.8rem;

        position: fixed;
        top: 0;
        width: 100%;
        backdrop-filter: blur(5px);
        /* background-color: blueviolet; */

        display: flex;
        justify-content: space-between;
        align-items: center;
    }
    .name {
        color: #f1f3f5;
    }

    ul {
        display: flex;
        justify-content: space-around;
    }
    ul li {
        margin: auto 1.6rem;
        list-style-type: none;
    }
    a {
        text-decoration: none;
    }

    .item:link,
    .item:visited {
        color: #f1f3f5;
    }

    .item:hover,
    .item:active {
        color: coral;
    }

    .item {
        width: 110%;
        display: flex;
        align-items: center;
        justify-content: space-between;
    }
</style>
