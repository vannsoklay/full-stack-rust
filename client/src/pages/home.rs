use crate::components::card::Card;
use wasm_bindgen::JsValue;
use yew::prelude::*;
use yewdux::log::info;
use crate::context::use_user_context;

#[function_component]
pub fn Home() -> Html { 
    let user_ctx = use_user_context();
    html! {
        <div class="grid lg:grid-cols-3 md:grid-cols-2 grid-cols-1 gap-5">
            <Card>
    
                <div class="h-48 w-full">
                    <img class="h-full w-full object-cover" src="https://images.squarespace-cdn.com/content/v1/5d9c6914c5dd9a58c2c4b394/1618650024957-NO105LK0OV6DD25W8J9P/IMG_8654.jpg?format=1000w"/>
                </div>
                <div class="flex space-x-2">
                    <ul class="h-11 w-11 rounded-full bg-gray-300">
                        <img class="h-full w-full rounded-full object-cover" src="https://assets.materialup.com/uploads/b78ca002-cd6c-4f84-befb-c09dd9261025/preview.png" />
                    </ul>
                    <ul>
                        <li class="text-base font-semibold text-gray-800">{"Title"}</li>
                        <li class="text-sm font-medium text-gray-600 cursor-pointer hover:text-gray-900">{"Vann Soklay"}</li>
                        <li class="text-sm font-medium text-gray-600"><label>{"100 Reading"}</label>{" • "}<label>{"4 months"}</label></li>
                    </ul>
                </div>
            </Card>
            <Card>
            <div class="h-48 w-full">
            <img class="h-full w-full object-cover" src="https://images.squarespace-cdn.com/content/v1/5d9c6914c5dd9a58c2c4b394/1618650024957-NO105LK0OV6DD25W8J9P/IMG_8654.jpg?format=1000w"/>
        </div>
        <div class="flex space-x-2">
            <ul class="h-11 w-11 rounded-full bg-gray-300">
                <img class="h-full w-full rounded-full object-cover" src="https://assets.materialup.com/uploads/b78ca002-cd6c-4f84-befb-c09dd9261025/preview.png" />
            </ul>
            <ul>
                <li class="text-base font-semibold text-gray-800">{"Title"}</li>
                <li class="text-sm font-medium text-gray-600 cursor-pointer hover:text-gray-900">{"Vann Soklay"}</li>
                <li class="text-sm font-medium text-gray-600"><label>{"100 Reading"}</label>{" • "}<label>{"4 months"}</label></li>
            </ul>
        </div>
            </Card>
            <Card>
            <div class="h-48 w-full">
            <img class="h-full w-full object-cover" src="https://images.squarespace-cdn.com/content/v1/5d9c6914c5dd9a58c2c4b394/1618650024957-NO105LK0OV6DD25W8J9P/IMG_8654.jpg?format=1000w"/>
        </div>
        <div class="flex space-x-2">
            <ul class="h-11 w-11 rounded-full bg-gray-300">
                <img class="h-full w-full rounded-full object-cover" src="https://assets.materialup.com/uploads/b78ca002-cd6c-4f84-befb-c09dd9261025/preview.png" />
            </ul>
            <ul>
                <li class="text-base font-semibold text-gray-800">{"Title"}</li>
                <li class="text-sm font-medium text-gray-600 cursor-pointer hover:text-gray-900">{"Vann Soklay"}</li>
                <li class="text-sm font-medium text-gray-600"><label>{"100 Reading"}</label>{" • "}<label>{"4 months"}</label></li>
            </ul>
        </div>
            </Card>
            <Card>
            <div class="h-48 w-full">
            <img class="h-full w-full object-cover" src="https://images.squarespace-cdn.com/content/v1/5d9c6914c5dd9a58c2c4b394/1618650024957-NO105LK0OV6DD25W8J9P/IMG_8654.jpg?format=1000w"/>
        </div>
        <div class="flex space-x-2">
            <ul class="h-11 w-11 rounded-full bg-gray-300">
                <img class="h-full w-full rounded-full object-cover" src="https://assets.materialup.com/uploads/b78ca002-cd6c-4f84-befb-c09dd9261025/preview.png" />
            </ul>
            <ul>
                <li class="text-base font-semibold text-gray-800">{"Title"}</li>
                <li class="text-sm font-medium text-gray-600 cursor-pointer hover:text-gray-900">{"Vann Soklay"}</li>
                <li class="text-sm font-medium text-gray-600"><label>{"100 Reading"}</label>{" • "}<label>{"4 months"}</label></li>
            </ul>
        </div>
            </Card>
        </div>
    }
}
