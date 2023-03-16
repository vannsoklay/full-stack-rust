use yew::prelude::*;

#[function_component]
pub fn Login() -> Html {
    html! {
         <div class="bg-grey-lighter font-sans">
            <main class="container mx-auto flex justify-center items-center">
                <div class="w-1/3">
                    <div class="font-hairline flex justify-center w-full">
                        <img src="static/assets/naruto.png" alt="iron-man"/>
                    </div>
                    <div class="p-8 bg-white mb-4">
                        <div class="mb-4">
                            <label class="text-gray-600 text-sm font-bold text-grey-darker block mb-2">{"Username or Email"}</label>
                            <input class="text-gray-500 block text-sm appearance-none rounded-sm w-full bg-white border border-grey-light hover:border-grey px-2 py-2 outline-none" type="text" placeholder="Your Username"/>
                        </div>
                        <div class="mb-4">
                            <label class="text-gray-600 text-sm font-bold text-grey-darker block mb-2">{"Password"}</label>
                            <input class="text-gray-500 block appearance-none w-full rounded-sm text-sm bg-white border border-grey-light hover:border-grey px-2 py-2 outline-none" type="password" placeholder="Your Password"/>
                        </div>
                        <div class="flex items-center justify-between mt-6">
                            <button class="bg-blue-500 hover:bg-blue-600 text-white w-full font-bold py-2 px-4 rounded-sm text-base">
                                {"Login"}
                            </button>
                        </div>
                    </div>
                </div>
            </main>
        </div>
    }
}
