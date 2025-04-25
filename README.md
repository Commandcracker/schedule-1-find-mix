# Find Mix

**Find Mix** is a brute force tool designed to calculate the best combination of ingredients to maximize profit for various products. 

**🚧 Work in process!**

## Performance Features

- Threading (with rayon)
- Allocating (with mimalloc)
- Some general optimizations like reduced use of clone etc.

## Example

![](0_8.png)

$ find_mix 0 8

Product: OG Kush \
Profit: \$144.20001 \
Cost: \$35 \
Sell Price: \$179.20001 \
Ingredients: Banana → Cuke → Gasoline → Horse Semen → Cuke → Mega Bean → Viagra → Mouth Wash \
Effects: <span style="color:rgb(133, 228, 89)">Glowing</span>, <span style="color:rgb(85, 200, 253)">Electrifying</span>, <span style="color:rgb(35, 91, 203)">Anti-Gravity</span>, <span style="color:rgb(190, 247, 253)">Bright-Eyed</span>, <span style="color:rgb(254, 217, 97)">Long Faced</span>, <span style="color:rgb(254, 193, 116)">Cyclopean</span>, <span style="color:rgb(176, 176, 175)">Foggy</span>, <span style="color:rgb(254, 159, 71)">Tropic Thunder</span>, <span style="color:rgb(199, 146, 50)">Balding</span> \
Execution Time: 29.0604887 seconds

## Todos

- Add reverse calculator
- Make it so ingredients can be disabled
- Add rounding and fix floating point issues (only use int?)

## Products

| ID | Product Name     | Base Value | Bse Effect                                               |
|----|------------------|------------|----------------------------------------------------------|
| 0  | OG Kush          | $35        | <span style="color:rgb(254, 208, 15)">Calming</span>     |
| 1  | Sour Diesel      | $35        | <span style="color:rgb(178, 254, 152)">Refreshing</span> |
| 2  | Green Crack      | $35        | <span style="color:rgb(154, 254, 109)">Energizing</span> |
| 3  | Grandaddy Purple | $35        | <span style="color:rgb(107, 95, 216)">Sedating</span>    |
| 4  | Meth             | $70        | None                                                     |
| 5  | Coke             | $150       | None                                                     |

# Ingredients

$\text{Cost}=\sum\text{Ingredient Prices}$

| Ingredient   | Price |
|--------------|-------|
| Cuke         | $2    |
| Banana       | $2    |
| Paracetamol  | $3    |
| Donut        | $3    |
| Viagra       | $4    |
| Mouth Wash   | $4    |
| Flu Medicine | $5    |
| Gasoline     | $5    |
| Energy Drink | $6    |
| Motor Oil    | $6    |
| Mega Bean    | $7    |
| Chili        | $7    |
| Battery      | $8    |
| Iodine       | $8    |
| Addy         | $9    |
| Horse Semen  | $9    |

## Effects

$\text{Sell Price}=\min(\max(\text{Base Value}\times(1+\sum\text{Effect Multipliers}),1),999)$ \
$\text{Profit}=\text{Sell Price}-\text{Cost}$

| Effect                                                          | Multiplier |
|-----------------------------------------------------------------|------------|
| <span style="color:rgb(182, 254, 218)">Shrinking</span>         | 0.60       |
| <span style="color:rgb(113, 171, 93)">Zombifying</span>         | 0.58       |
| <span style="color:rgb(254, 193, 116)">Cyclopean</span>         | 0.56       |
| <span style="color:rgb(35, 91, 203)">Anti-Gravity</span>        | 0.54       |
| <span style="color:rgb(254, 217, 97)">Long Faced</span>         | 0.52       |
| <span style="color:rgb(85, 200, 253)">Electrifying</span>       | 0.50       |
| <span style="color:rgb(133, 228, 89)">Glowing</span>            | 0.48       |
| <span style="color:rgb(254, 159, 71)">Tropic Thunder</span>     | 0.46       |
| <span style="color:rgb(254, 160, 203)">Thought-Provoking</span> | 0.44       |
| <span style="color:rgb(254, 141, 248)">Jennerising</span>       | 0.42       |
| <span style="color:rgb(190, 247, 253)">Bright-Eyed</span>       | 0.40       |
| <span style="color:rgb(254, 107, 76)">Spicy</span>              | 0.38       |
| <span style="color:rgb(176, 176, 175)">Foggy</span>             | 0.36       |
| <span style="color:rgb(162, 223, 253)">Slippery</span>          | 0.34       |
| <span style="color:rgb(117, 200, 253)">Athletic</span>          | 0.32       |
| <span style="color:rgb(199, 146, 50)">Balding</span>            | 0.30       |
| <span style="color:rgb(254, 132, 244)">Calorie-Dense</span>     | 0.28       |
| <span style="color:rgb(107, 95, 216)">Sedating</span>           | 0.26       |
| <span style="color:rgb(123, 123, 123)">Sneaky</span>            | 0.24       |
| <span style="color:rgb(154, 254, 109)">Energizing</span>        | 0.22       |
| <span style="color:rgb(254, 136, 41)">Gingeritis</span>         | 0.20       |
| <span style="color:rgb(254, 234, 116)">Euphoric</span>          | 0.18       |
| <span style="color:rgb(117, 241, 253)">Focused</span>           | 0.16       |
| <span style="color:rgb(178, 254, 152)">Refreshing</span>        | 0.14       |
| <span style="color:rgb(201, 110, 87)">Munchies</span>           | 0.12       |
| <span style="color:rgb(254, 208, 15)">Calming</span>            | 0.10       |
| <span style="color:rgb(254, 117, 81)">Disorienting</span>       | 0.00       |
| <span style="color:rgb(254, 75, 64)">Explosive</span>           | 0.00       |
| <span style="color:rgb(118, 60, 37)">Laxative</span>            | 0.00       |
| <span style="color:rgb(196, 103, 98)">Paranoia</span>           | 0.00       |
| <span style="color:rgb(100, 90, 253)">Schizophrenic</span>      | 0.00       |
| <span style="color:rgb(9254, 233, 0)">Seizure-Inducing</span>   | 0.00       |
| <span style="color:rgb(125, 188, 49)">Smelly</span>             | 0.00       |
| <span style="color:rgb(95, 154, 49)">Toxic</span>               | 0.00       |

## Effects Additives

$\text{Additives}=\min(\max(\sum\text{Effect Additives}+0.05\cdot\mathbb{1}_{\sum\text{Ingredients}>0},0),1)$ \
When Displayed: $\text{floor}\lfloor\text{Additives}*100\rfloor$

| Effect                                                          | Additives |
|-----------------------------------------------------------------|-----------|
| <span style="color:rgb(254, 159, 71)">Tropic Thunder</span>     | 0.803     |
| <span style="color:rgb(254, 107, 76)">Spicy</span>              | 0.665     |
| <span style="color:rgb(35, 91, 203)">Anti-Gravity</span>        | 0.611     |
| <span style="color:rgb(117, 200, 253)">Athletic</span>          | 0.607     |
| <span style="color:rgb(254, 217, 97)">Long Faced</span>         | 0.607     |
| <span style="color:rgb(113, 171, 93)">Zombifying</span>         | 0.598     |
| <span style="color:rgb(133, 228, 89)">Glowing</span>            | 0.472     |
| <span style="color:rgb(254, 160, 203)">Thought-Provoking</span> | 0.370     |
| <span style="color:rgb(254, 141, 248)">Jennerising</span>       | 0.343     |
| <span style="color:rgb(154, 254, 109)">Energizing</span>        | 0.340     |
| <span style="color:rgb(182, 254, 218)">Shrinking</span>         | 0.336     |
| <span style="color:rgb(123, 123, 123)">Sneaky</span>            | 0.327     |
| <span style="color:rgb(162, 223, 253)">Slippery</span>          | 0.309     |
| <span style="color:rgb(254, 234, 116)">Euphoric</span>          | 0.235     |
| <span style="color:rgb(85, 200, 253)">Electrifying</span>       | 0.235     |
| <span style="color:rgb(190, 247, 253)">Bright-Eyed</span>       | 0.200     |
| <span style="color:rgb(117, 241, 253)">Focused</span>           | 0.104     |
| <span style="color:rgb(178, 254, 152)">Refreshing</span>        | 0.104     |
| <span style="color:rgb(254, 132, 244)">Calorie-Dense</span>     | 0.100     |
| <span style="color:rgb(254, 193, 116)">Cyclopean</span>         | 0.100     |
| <span style="color:rgb(176, 176, 175)">Foggy</span>             | 0.100     |
| <span style="color:rgb(118, 60, 37)">Laxative</span>            | 0.100     |
| <span style="color:rgb(201, 110, 87)">Munchies</span>           | 0.096     |
| <span style="color:rgb(254, 136, 41)">Gingeritis</span>         | 0.000     |
| <span style="color:rgb(107, 95, 216)">Sedating</span>           | 0.000     |
| <span style="color:rgb(199, 146, 50)">Balding</span>            | 0.000     |
| <span style="color:rgb(254, 208, 15)">Calming</span>            | 0.000     |
| <span style="color:rgb(254, 117, 81)">Disorienting</span>       | 0.000     |
| <span style="color:rgb(254, 75, 64)">Explosive</span>           | 0.000     |
| <span style="color:rgb(196, 103, 98)">Paranoia</span>           | 0.000     |
| <span style="color:rgb(100, 90, 253)">Schizophrenic</span>      | 0.000     |
| <span style="color:rgb(9254, 233, 0)">Seizure-Inducing</span>   | 0.000     |
| <span style="color:rgb(125, 188, 49)">Smelly</span>             | 0.000     |
| <span style="color:rgb(95, 154, 49)">Toxic</span>               | 0.000     |
