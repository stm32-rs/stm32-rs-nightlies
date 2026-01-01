///Register `GPIOB_BRR` writer
pub type W = crate::W<GPIOB_BRRrs>;
/**Port x reset I/O y These bits are write-only. A read operation always returns 0x0000.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR0 {
    ///0: No action on the corresponding ODx bit
    B0x0 = 0,
    ///1: Reset the corresponding ODx bit
    B0x1 = 1,
}
impl From<BR0> for bool {
    #[inline(always)]
    fn from(variant: BR0) -> Self {
        variant as u8 != 0
    }
}
///Field `BR0` writer - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000.
pub type BR0_W<'a, REG> = crate::BitWriter<'a, REG, BR0>;
impl<'a, REG> BR0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action on the corresponding ODx bit
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BR0::B0x0)
    }
    ///Reset the corresponding ODx bit
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR0::B0x1)
    }
}
/**Port x reset I/O y These bits are write-only. A read operation always returns 0x0000.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR1 {
    ///0: No action on the corresponding ODx bit
    B0x0 = 0,
    ///1: Reset the corresponding ODx bit
    B0x1 = 1,
}
impl From<BR1> for bool {
    #[inline(always)]
    fn from(variant: BR1) -> Self {
        variant as u8 != 0
    }
}
///Field `BR1` writer - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000.
pub type BR1_W<'a, REG> = crate::BitWriter<'a, REG, BR1>;
impl<'a, REG> BR1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action on the corresponding ODx bit
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BR1::B0x0)
    }
    ///Reset the corresponding ODx bit
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR1::B0x1)
    }
}
/**Port x reset I/O y These bits are write-only. A read operation always returns 0x0000.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR2 {
    ///0: No action on the corresponding ODx bit
    B0x0 = 0,
    ///1: Reset the corresponding ODx bit
    B0x1 = 1,
}
impl From<BR2> for bool {
    #[inline(always)]
    fn from(variant: BR2) -> Self {
        variant as u8 != 0
    }
}
///Field `BR2` writer - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000.
pub type BR2_W<'a, REG> = crate::BitWriter<'a, REG, BR2>;
impl<'a, REG> BR2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action on the corresponding ODx bit
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BR2::B0x0)
    }
    ///Reset the corresponding ODx bit
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR2::B0x1)
    }
}
/**Port x reset I/O y These bits are write-only. A read operation always returns 0x0000.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR3 {
    ///0: No action on the corresponding ODx bit
    B0x0 = 0,
    ///1: Reset the corresponding ODx bit
    B0x1 = 1,
}
impl From<BR3> for bool {
    #[inline(always)]
    fn from(variant: BR3) -> Self {
        variant as u8 != 0
    }
}
///Field `BR3` writer - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000.
pub type BR3_W<'a, REG> = crate::BitWriter<'a, REG, BR3>;
impl<'a, REG> BR3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action on the corresponding ODx bit
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BR3::B0x0)
    }
    ///Reset the corresponding ODx bit
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR3::B0x1)
    }
}
/**Port x reset I/O y These bits are write-only. A read operation always returns 0x0000.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR4 {
    ///0: No action on the corresponding ODx bit
    B0x0 = 0,
    ///1: Reset the corresponding ODx bit
    B0x1 = 1,
}
impl From<BR4> for bool {
    #[inline(always)]
    fn from(variant: BR4) -> Self {
        variant as u8 != 0
    }
}
///Field `BR4` writer - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000.
pub type BR4_W<'a, REG> = crate::BitWriter<'a, REG, BR4>;
impl<'a, REG> BR4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action on the corresponding ODx bit
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BR4::B0x0)
    }
    ///Reset the corresponding ODx bit
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR4::B0x1)
    }
}
/**Port x reset I/O y These bits are write-only. A read operation always returns 0x0000.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR5 {
    ///0: No action on the corresponding ODx bit
    B0x0 = 0,
    ///1: Reset the corresponding ODx bit
    B0x1 = 1,
}
impl From<BR5> for bool {
    #[inline(always)]
    fn from(variant: BR5) -> Self {
        variant as u8 != 0
    }
}
///Field `BR5` writer - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000.
pub type BR5_W<'a, REG> = crate::BitWriter<'a, REG, BR5>;
impl<'a, REG> BR5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action on the corresponding ODx bit
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BR5::B0x0)
    }
    ///Reset the corresponding ODx bit
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR5::B0x1)
    }
}
/**Port x reset I/O y These bits are write-only. A read operation always returns 0x0000.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR6 {
    ///0: No action on the corresponding ODx bit
    B0x0 = 0,
    ///1: Reset the corresponding ODx bit
    B0x1 = 1,
}
impl From<BR6> for bool {
    #[inline(always)]
    fn from(variant: BR6) -> Self {
        variant as u8 != 0
    }
}
///Field `BR6` writer - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000.
pub type BR6_W<'a, REG> = crate::BitWriter<'a, REG, BR6>;
impl<'a, REG> BR6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action on the corresponding ODx bit
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BR6::B0x0)
    }
    ///Reset the corresponding ODx bit
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR6::B0x1)
    }
}
/**Port x reset I/O y These bits are write-only. A read operation always returns 0x0000.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR7 {
    ///0: No action on the corresponding ODx bit
    B0x0 = 0,
    ///1: Reset the corresponding ODx bit
    B0x1 = 1,
}
impl From<BR7> for bool {
    #[inline(always)]
    fn from(variant: BR7) -> Self {
        variant as u8 != 0
    }
}
///Field `BR7` writer - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000.
pub type BR7_W<'a, REG> = crate::BitWriter<'a, REG, BR7>;
impl<'a, REG> BR7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action on the corresponding ODx bit
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BR7::B0x0)
    }
    ///Reset the corresponding ODx bit
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR7::B0x1)
    }
}
/**Port x reset I/O y These bits are write-only. A read operation always returns 0x0000.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR8 {
    ///0: No action on the corresponding ODx bit
    B0x0 = 0,
    ///1: Reset the corresponding ODx bit
    B0x1 = 1,
}
impl From<BR8> for bool {
    #[inline(always)]
    fn from(variant: BR8) -> Self {
        variant as u8 != 0
    }
}
///Field `BR8` writer - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000.
pub type BR8_W<'a, REG> = crate::BitWriter<'a, REG, BR8>;
impl<'a, REG> BR8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action on the corresponding ODx bit
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BR8::B0x0)
    }
    ///Reset the corresponding ODx bit
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR8::B0x1)
    }
}
/**Port x reset I/O y These bits are write-only. A read operation always returns 0x0000.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR9 {
    ///0: No action on the corresponding ODx bit
    B0x0 = 0,
    ///1: Reset the corresponding ODx bit
    B0x1 = 1,
}
impl From<BR9> for bool {
    #[inline(always)]
    fn from(variant: BR9) -> Self {
        variant as u8 != 0
    }
}
///Field `BR9` writer - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000.
pub type BR9_W<'a, REG> = crate::BitWriter<'a, REG, BR9>;
impl<'a, REG> BR9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action on the corresponding ODx bit
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BR9::B0x0)
    }
    ///Reset the corresponding ODx bit
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR9::B0x1)
    }
}
/**Port x reset I/O y These bits are write-only. A read operation always returns 0x0000.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR10 {
    ///0: No action on the corresponding ODx bit
    B0x0 = 0,
    ///1: Reset the corresponding ODx bit
    B0x1 = 1,
}
impl From<BR10> for bool {
    #[inline(always)]
    fn from(variant: BR10) -> Self {
        variant as u8 != 0
    }
}
///Field `BR10` writer - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000.
pub type BR10_W<'a, REG> = crate::BitWriter<'a, REG, BR10>;
impl<'a, REG> BR10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action on the corresponding ODx bit
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BR10::B0x0)
    }
    ///Reset the corresponding ODx bit
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR10::B0x1)
    }
}
/**Port x reset I/O y These bits are write-only. A read operation always returns 0x0000.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR11 {
    ///0: No action on the corresponding ODx bit
    B0x0 = 0,
    ///1: Reset the corresponding ODx bit
    B0x1 = 1,
}
impl From<BR11> for bool {
    #[inline(always)]
    fn from(variant: BR11) -> Self {
        variant as u8 != 0
    }
}
///Field `BR11` writer - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000.
pub type BR11_W<'a, REG> = crate::BitWriter<'a, REG, BR11>;
impl<'a, REG> BR11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action on the corresponding ODx bit
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BR11::B0x0)
    }
    ///Reset the corresponding ODx bit
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR11::B0x1)
    }
}
/**Port x reset I/O y These bits are write-only. A read operation always returns 0x0000.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR12 {
    ///0: No action on the corresponding ODx bit
    B0x0 = 0,
    ///1: Reset the corresponding ODx bit
    B0x1 = 1,
}
impl From<BR12> for bool {
    #[inline(always)]
    fn from(variant: BR12) -> Self {
        variant as u8 != 0
    }
}
///Field `BR12` writer - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000.
pub type BR12_W<'a, REG> = crate::BitWriter<'a, REG, BR12>;
impl<'a, REG> BR12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action on the corresponding ODx bit
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BR12::B0x0)
    }
    ///Reset the corresponding ODx bit
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR12::B0x1)
    }
}
/**Port x reset I/O y These bits are write-only. A read operation always returns 0x0000.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR13 {
    ///0: No action on the corresponding ODx bit
    B0x0 = 0,
    ///1: Reset the corresponding ODx bit
    B0x1 = 1,
}
impl From<BR13> for bool {
    #[inline(always)]
    fn from(variant: BR13) -> Self {
        variant as u8 != 0
    }
}
///Field `BR13` writer - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000.
pub type BR13_W<'a, REG> = crate::BitWriter<'a, REG, BR13>;
impl<'a, REG> BR13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action on the corresponding ODx bit
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BR13::B0x0)
    }
    ///Reset the corresponding ODx bit
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR13::B0x1)
    }
}
/**Port x reset I/O y These bits are write-only. A read operation always returns 0x0000.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR14 {
    ///0: No action on the corresponding ODx bit
    B0x0 = 0,
    ///1: Reset the corresponding ODx bit
    B0x1 = 1,
}
impl From<BR14> for bool {
    #[inline(always)]
    fn from(variant: BR14) -> Self {
        variant as u8 != 0
    }
}
///Field `BR14` writer - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000.
pub type BR14_W<'a, REG> = crate::BitWriter<'a, REG, BR14>;
impl<'a, REG> BR14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action on the corresponding ODx bit
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BR14::B0x0)
    }
    ///Reset the corresponding ODx bit
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR14::B0x1)
    }
}
/**Port x reset I/O y These bits are write-only. A read operation always returns 0x0000.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR15 {
    ///0: No action on the corresponding ODx bit
    B0x0 = 0,
    ///1: Reset the corresponding ODx bit
    B0x1 = 1,
}
impl From<BR15> for bool {
    #[inline(always)]
    fn from(variant: BR15) -> Self {
        variant as u8 != 0
    }
}
///Field `BR15` writer - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000.
pub type BR15_W<'a, REG> = crate::BitWriter<'a, REG, BR15>;
impl<'a, REG> BR15_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action on the corresponding ODx bit
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BR15::B0x0)
    }
    ///Reset the corresponding ODx bit
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR15::B0x1)
    }
}
impl core::fmt::Debug for crate::generic::Reg<GPIOB_BRRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000.
    #[inline(always)]
    pub fn br0(&mut self) -> BR0_W<'_, GPIOB_BRRrs> {
        BR0_W::new(self, 0)
    }
    ///Bit 1 - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000.
    #[inline(always)]
    pub fn br1(&mut self) -> BR1_W<'_, GPIOB_BRRrs> {
        BR1_W::new(self, 1)
    }
    ///Bit 2 - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000.
    #[inline(always)]
    pub fn br2(&mut self) -> BR2_W<'_, GPIOB_BRRrs> {
        BR2_W::new(self, 2)
    }
    ///Bit 3 - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000.
    #[inline(always)]
    pub fn br3(&mut self) -> BR3_W<'_, GPIOB_BRRrs> {
        BR3_W::new(self, 3)
    }
    ///Bit 4 - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000.
    #[inline(always)]
    pub fn br4(&mut self) -> BR4_W<'_, GPIOB_BRRrs> {
        BR4_W::new(self, 4)
    }
    ///Bit 5 - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000.
    #[inline(always)]
    pub fn br5(&mut self) -> BR5_W<'_, GPIOB_BRRrs> {
        BR5_W::new(self, 5)
    }
    ///Bit 6 - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000.
    #[inline(always)]
    pub fn br6(&mut self) -> BR6_W<'_, GPIOB_BRRrs> {
        BR6_W::new(self, 6)
    }
    ///Bit 7 - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000.
    #[inline(always)]
    pub fn br7(&mut self) -> BR7_W<'_, GPIOB_BRRrs> {
        BR7_W::new(self, 7)
    }
    ///Bit 8 - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000.
    #[inline(always)]
    pub fn br8(&mut self) -> BR8_W<'_, GPIOB_BRRrs> {
        BR8_W::new(self, 8)
    }
    ///Bit 9 - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000.
    #[inline(always)]
    pub fn br9(&mut self) -> BR9_W<'_, GPIOB_BRRrs> {
        BR9_W::new(self, 9)
    }
    ///Bit 10 - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000.
    #[inline(always)]
    pub fn br10(&mut self) -> BR10_W<'_, GPIOB_BRRrs> {
        BR10_W::new(self, 10)
    }
    ///Bit 11 - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000.
    #[inline(always)]
    pub fn br11(&mut self) -> BR11_W<'_, GPIOB_BRRrs> {
        BR11_W::new(self, 11)
    }
    ///Bit 12 - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000.
    #[inline(always)]
    pub fn br12(&mut self) -> BR12_W<'_, GPIOB_BRRrs> {
        BR12_W::new(self, 12)
    }
    ///Bit 13 - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000.
    #[inline(always)]
    pub fn br13(&mut self) -> BR13_W<'_, GPIOB_BRRrs> {
        BR13_W::new(self, 13)
    }
    ///Bit 14 - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000.
    #[inline(always)]
    pub fn br14(&mut self) -> BR14_W<'_, GPIOB_BRRrs> {
        BR14_W::new(self, 14)
    }
    ///Bit 15 - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000.
    #[inline(always)]
    pub fn br15(&mut self) -> BR15_W<'_, GPIOB_BRRrs> {
        BR15_W::new(self, 15)
    }
}
/**GPIO port bit reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_brr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#GPIOB:GPIOB_BRR)*/
pub struct GPIOB_BRRrs;
impl crate::RegisterSpec for GPIOB_BRRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`gpiob_brr::W`](W) writer structure
impl crate::Writable for GPIOB_BRRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GPIOB_BRR to value 0
impl crate::Resettable for GPIOB_BRRrs {}
