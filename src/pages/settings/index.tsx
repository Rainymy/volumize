import style from "./index.module.less";

export function Settings() {
    return (
        <div className={style.container}>
            <h1>Settings</h1>

            <form className={style.settings}>
                <div>
                    <input type="checkbox" />
                    <span>
                        Discovert from last session
                    </span>
                </div>
                <div>
                    <input type="checkbox" />
                    <span>
                        Retry count
                    </span>
                </div>
                <div>
                    <input type="number" min={0} max={60} />
                    <span>
                        Timeout duration (seconds)
                    </span>
                </div>
                <button type="submit">Save</button>
            </form>
        </div>
    );
}