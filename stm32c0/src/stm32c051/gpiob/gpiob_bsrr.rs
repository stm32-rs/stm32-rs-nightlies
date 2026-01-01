///Register `GPIOB_BSRR` writer
pub type W = crate::W<GPIOB_BSRRrs>;
/**Port x set I/O y These bits are write-only. A read operation always returns 0x0000.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BS0 {
    ///0: No action on the corresponding ODRx bit
    B0x0 = 0,
    ///1: Sets the corresponding ODRx bit
    B0x1 = 1,
}
impl From<BS0> for bool {
    #[inline(always)]
    fn from(variant: BS0) -> Self {
        variant as u8 != 0
    }
}
///Field `BS0` writer - Port x set I/O y These bits are write-only. A read operation always returns 0x0000.
pub type BS0_W<'a, REG> = crate::BitWriter<'a, REG, BS0>;
impl<'a, REG> BS0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action on the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BS0::B0x0)
    }
    ///Sets the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BS0::B0x1)
    }
}
/**Port x set I/O y These bits are write-only. A read operation always returns 0x0000.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BS1 {
    ///0: No action on the corresponding ODRx bit
    B0x0 = 0,
    ///1: Sets the corresponding ODRx bit
    B0x1 = 1,
}
impl From<BS1> for bool {
    #[inline(always)]
    fn from(variant: BS1) -> Self {
        variant as u8 != 0
    }
}
///Field `BS1` writer - Port x set I/O y These bits are write-only. A read operation always returns 0x0000.
pub type BS1_W<'a, REG> = crate::BitWriter<'a, REG, BS1>;
impl<'a, REG> BS1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action on the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BS1::B0x0)
    }
    ///Sets the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BS1::B0x1)
    }
}
/**Port x set I/O y These bits are write-only. A read operation always returns 0x0000.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BS2 {
    ///0: No action on the corresponding ODRx bit
    B0x0 = 0,
    ///1: Sets the corresponding ODRx bit
    B0x1 = 1,
}
impl From<BS2> for bool {
    #[inline(always)]
    fn from(variant: BS2) -> Self {
        variant as u8 != 0
    }
}
///Field `BS2` writer - Port x set I/O y These bits are write-only. A read operation always returns 0x0000.
pub type BS2_W<'a, REG> = crate::BitWriter<'a, REG, BS2>;
impl<'a, REG> BS2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action on the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BS2::B0x0)
    }
    ///Sets the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BS2::B0x1)
    }
}
/**Port x set I/O y These bits are write-only. A read operation always returns 0x0000.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BS3 {
    ///0: No action on the corresponding ODRx bit
    B0x0 = 0,
    ///1: Sets the corresponding ODRx bit
    B0x1 = 1,
}
impl From<BS3> for bool {
    #[inline(always)]
    fn from(variant: BS3) -> Self {
        variant as u8 != 0
    }
}
///Field `BS3` writer - Port x set I/O y These bits are write-only. A read operation always returns 0x0000.
pub type BS3_W<'a, REG> = crate::BitWriter<'a, REG, BS3>;
impl<'a, REG> BS3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action on the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BS3::B0x0)
    }
    ///Sets the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BS3::B0x1)
    }
}
/**Port x set I/O y These bits are write-only. A read operation always returns 0x0000.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BS4 {
    ///0: No action on the corresponding ODRx bit
    B0x0 = 0,
    ///1: Sets the corresponding ODRx bit
    B0x1 = 1,
}
impl From<BS4> for bool {
    #[inline(always)]
    fn from(variant: BS4) -> Self {
        variant as u8 != 0
    }
}
///Field `BS4` writer - Port x set I/O y These bits are write-only. A read operation always returns 0x0000.
pub type BS4_W<'a, REG> = crate::BitWriter<'a, REG, BS4>;
impl<'a, REG> BS4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action on the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BS4::B0x0)
    }
    ///Sets the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BS4::B0x1)
    }
}
/**Port x set I/O y These bits are write-only. A read operation always returns 0x0000.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BS5 {
    ///0: No action on the corresponding ODRx bit
    B0x0 = 0,
    ///1: Sets the corresponding ODRx bit
    B0x1 = 1,
}
impl From<BS5> for bool {
    #[inline(always)]
    fn from(variant: BS5) -> Self {
        variant as u8 != 0
    }
}
///Field `BS5` writer - Port x set I/O y These bits are write-only. A read operation always returns 0x0000.
pub type BS5_W<'a, REG> = crate::BitWriter<'a, REG, BS5>;
impl<'a, REG> BS5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action on the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BS5::B0x0)
    }
    ///Sets the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BS5::B0x1)
    }
}
/**Port x set I/O y These bits are write-only. A read operation always returns 0x0000.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BS6 {
    ///0: No action on the corresponding ODRx bit
    B0x0 = 0,
    ///1: Sets the corresponding ODRx bit
    B0x1 = 1,
}
impl From<BS6> for bool {
    #[inline(always)]
    fn from(variant: BS6) -> Self {
        variant as u8 != 0
    }
}
///Field `BS6` writer - Port x set I/O y These bits are write-only. A read operation always returns 0x0000.
pub type BS6_W<'a, REG> = crate::BitWriter<'a, REG, BS6>;
impl<'a, REG> BS6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action on the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BS6::B0x0)
    }
    ///Sets the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BS6::B0x1)
    }
}
/**Port x set I/O y These bits are write-only. A read operation always returns 0x0000.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BS7 {
    ///0: No action on the corresponding ODRx bit
    B0x0 = 0,
    ///1: Sets the corresponding ODRx bit
    B0x1 = 1,
}
impl From<BS7> for bool {
    #[inline(always)]
    fn from(variant: BS7) -> Self {
        variant as u8 != 0
    }
}
///Field `BS7` writer - Port x set I/O y These bits are write-only. A read operation always returns 0x0000.
pub type BS7_W<'a, REG> = crate::BitWriter<'a, REG, BS7>;
impl<'a, REG> BS7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action on the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BS7::B0x0)
    }
    ///Sets the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BS7::B0x1)
    }
}
/**Port x set I/O y These bits are write-only. A read operation always returns 0x0000.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BS8 {
    ///0: No action on the corresponding ODRx bit
    B0x0 = 0,
    ///1: Sets the corresponding ODRx bit
    B0x1 = 1,
}
impl From<BS8> for bool {
    #[inline(always)]
    fn from(variant: BS8) -> Self {
        variant as u8 != 0
    }
}
///Field `BS8` writer - Port x set I/O y These bits are write-only. A read operation always returns 0x0000.
pub type BS8_W<'a, REG> = crate::BitWriter<'a, REG, BS8>;
impl<'a, REG> BS8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action on the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BS8::B0x0)
    }
    ///Sets the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BS8::B0x1)
    }
}
/**Port x set I/O y These bits are write-only. A read operation always returns 0x0000.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BS9 {
    ///0: No action on the corresponding ODRx bit
    B0x0 = 0,
    ///1: Sets the corresponding ODRx bit
    B0x1 = 1,
}
impl From<BS9> for bool {
    #[inline(always)]
    fn from(variant: BS9) -> Self {
        variant as u8 != 0
    }
}
///Field `BS9` writer - Port x set I/O y These bits are write-only. A read operation always returns 0x0000.
pub type BS9_W<'a, REG> = crate::BitWriter<'a, REG, BS9>;
impl<'a, REG> BS9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action on the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BS9::B0x0)
    }
    ///Sets the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BS9::B0x1)
    }
}
/**Port x set I/O y These bits are write-only. A read operation always returns 0x0000.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BS10 {
    ///0: No action on the corresponding ODRx bit
    B0x0 = 0,
    ///1: Sets the corresponding ODRx bit
    B0x1 = 1,
}
impl From<BS10> for bool {
    #[inline(always)]
    fn from(variant: BS10) -> Self {
        variant as u8 != 0
    }
}
///Field `BS10` writer - Port x set I/O y These bits are write-only. A read operation always returns 0x0000.
pub type BS10_W<'a, REG> = crate::BitWriter<'a, REG, BS10>;
impl<'a, REG> BS10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action on the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BS10::B0x0)
    }
    ///Sets the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BS10::B0x1)
    }
}
/**Port x set I/O y These bits are write-only. A read operation always returns 0x0000.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BS11 {
    ///0: No action on the corresponding ODRx bit
    B0x0 = 0,
    ///1: Sets the corresponding ODRx bit
    B0x1 = 1,
}
impl From<BS11> for bool {
    #[inline(always)]
    fn from(variant: BS11) -> Self {
        variant as u8 != 0
    }
}
///Field `BS11` writer - Port x set I/O y These bits are write-only. A read operation always returns 0x0000.
pub type BS11_W<'a, REG> = crate::BitWriter<'a, REG, BS11>;
impl<'a, REG> BS11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action on the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BS11::B0x0)
    }
    ///Sets the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BS11::B0x1)
    }
}
/**Port x set I/O y These bits are write-only. A read operation always returns 0x0000.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BS12 {
    ///0: No action on the corresponding ODRx bit
    B0x0 = 0,
    ///1: Sets the corresponding ODRx bit
    B0x1 = 1,
}
impl From<BS12> for bool {
    #[inline(always)]
    fn from(variant: BS12) -> Self {
        variant as u8 != 0
    }
}
///Field `BS12` writer - Port x set I/O y These bits are write-only. A read operation always returns 0x0000.
pub type BS12_W<'a, REG> = crate::BitWriter<'a, REG, BS12>;
impl<'a, REG> BS12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action on the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BS12::B0x0)
    }
    ///Sets the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BS12::B0x1)
    }
}
/**Port x set I/O y These bits are write-only. A read operation always returns 0x0000.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BS13 {
    ///0: No action on the corresponding ODRx bit
    B0x0 = 0,
    ///1: Sets the corresponding ODRx bit
    B0x1 = 1,
}
impl From<BS13> for bool {
    #[inline(always)]
    fn from(variant: BS13) -> Self {
        variant as u8 != 0
    }
}
///Field `BS13` writer - Port x set I/O y These bits are write-only. A read operation always returns 0x0000.
pub type BS13_W<'a, REG> = crate::BitWriter<'a, REG, BS13>;
impl<'a, REG> BS13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action on the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BS13::B0x0)
    }
    ///Sets the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BS13::B0x1)
    }
}
/**Port x set I/O y These bits are write-only. A read operation always returns 0x0000.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BS14 {
    ///0: No action on the corresponding ODRx bit
    B0x0 = 0,
    ///1: Sets the corresponding ODRx bit
    B0x1 = 1,
}
impl From<BS14> for bool {
    #[inline(always)]
    fn from(variant: BS14) -> Self {
        variant as u8 != 0
    }
}
///Field `BS14` writer - Port x set I/O y These bits are write-only. A read operation always returns 0x0000.
pub type BS14_W<'a, REG> = crate::BitWriter<'a, REG, BS14>;
impl<'a, REG> BS14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action on the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BS14::B0x0)
    }
    ///Sets the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BS14::B0x1)
    }
}
/**Port x set I/O y These bits are write-only. A read operation always returns 0x0000.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BS15 {
    ///0: No action on the corresponding ODRx bit
    B0x0 = 0,
    ///1: Sets the corresponding ODRx bit
    B0x1 = 1,
}
impl From<BS15> for bool {
    #[inline(always)]
    fn from(variant: BS15) -> Self {
        variant as u8 != 0
    }
}
///Field `BS15` writer - Port x set I/O y These bits are write-only. A read operation always returns 0x0000.
pub type BS15_W<'a, REG> = crate::BitWriter<'a, REG, BS15>;
impl<'a, REG> BS15_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action on the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BS15::B0x0)
    }
    ///Sets the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BS15::B0x1)
    }
}
/**Port x reset I/O y These bits are write-only. A read operation always returns 0x0000. Note: If both BSx and BRx are set, BSx has priority.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR0 {
    ///0: No action on the corresponding ODRx bit
    B0x0 = 0,
    ///1: Resets the corresponding ODRx bit
    B0x1 = 1,
}
impl From<BR0> for bool {
    #[inline(always)]
    fn from(variant: BR0) -> Self {
        variant as u8 != 0
    }
}
///Field `BR0` writer - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000. Note: If both BSx and BRx are set, BSx has priority.
pub type BR0_W<'a, REG> = crate::BitWriter<'a, REG, BR0>;
impl<'a, REG> BR0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action on the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BR0::B0x0)
    }
    ///Resets the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR0::B0x1)
    }
}
/**Port x reset I/O y These bits are write-only. A read operation always returns 0x0000. Note: If both BSx and BRx are set, BSx has priority.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR1 {
    ///0: No action on the corresponding ODRx bit
    B0x0 = 0,
    ///1: Resets the corresponding ODRx bit
    B0x1 = 1,
}
impl From<BR1> for bool {
    #[inline(always)]
    fn from(variant: BR1) -> Self {
        variant as u8 != 0
    }
}
///Field `BR1` writer - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000. Note: If both BSx and BRx are set, BSx has priority.
pub type BR1_W<'a, REG> = crate::BitWriter<'a, REG, BR1>;
impl<'a, REG> BR1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action on the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BR1::B0x0)
    }
    ///Resets the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR1::B0x1)
    }
}
/**Port x reset I/O y These bits are write-only. A read operation always returns 0x0000. Note: If both BSx and BRx are set, BSx has priority.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR2 {
    ///0: No action on the corresponding ODRx bit
    B0x0 = 0,
    ///1: Resets the corresponding ODRx bit
    B0x1 = 1,
}
impl From<BR2> for bool {
    #[inline(always)]
    fn from(variant: BR2) -> Self {
        variant as u8 != 0
    }
}
///Field `BR2` writer - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000. Note: If both BSx and BRx are set, BSx has priority.
pub type BR2_W<'a, REG> = crate::BitWriter<'a, REG, BR2>;
impl<'a, REG> BR2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action on the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BR2::B0x0)
    }
    ///Resets the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR2::B0x1)
    }
}
/**Port x reset I/O y These bits are write-only. A read operation always returns 0x0000. Note: If both BSx and BRx are set, BSx has priority.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR3 {
    ///0: No action on the corresponding ODRx bit
    B0x0 = 0,
    ///1: Resets the corresponding ODRx bit
    B0x1 = 1,
}
impl From<BR3> for bool {
    #[inline(always)]
    fn from(variant: BR3) -> Self {
        variant as u8 != 0
    }
}
///Field `BR3` writer - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000. Note: If both BSx and BRx are set, BSx has priority.
pub type BR3_W<'a, REG> = crate::BitWriter<'a, REG, BR3>;
impl<'a, REG> BR3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action on the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BR3::B0x0)
    }
    ///Resets the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR3::B0x1)
    }
}
/**Port x reset I/O y These bits are write-only. A read operation always returns 0x0000. Note: If both BSx and BRx are set, BSx has priority.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR4 {
    ///0: No action on the corresponding ODRx bit
    B0x0 = 0,
    ///1: Resets the corresponding ODRx bit
    B0x1 = 1,
}
impl From<BR4> for bool {
    #[inline(always)]
    fn from(variant: BR4) -> Self {
        variant as u8 != 0
    }
}
///Field `BR4` writer - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000. Note: If both BSx and BRx are set, BSx has priority.
pub type BR4_W<'a, REG> = crate::BitWriter<'a, REG, BR4>;
impl<'a, REG> BR4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action on the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BR4::B0x0)
    }
    ///Resets the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR4::B0x1)
    }
}
/**Port x reset I/O y These bits are write-only. A read operation always returns 0x0000. Note: If both BSx and BRx are set, BSx has priority.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR5 {
    ///0: No action on the corresponding ODRx bit
    B0x0 = 0,
    ///1: Resets the corresponding ODRx bit
    B0x1 = 1,
}
impl From<BR5> for bool {
    #[inline(always)]
    fn from(variant: BR5) -> Self {
        variant as u8 != 0
    }
}
///Field `BR5` writer - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000. Note: If both BSx and BRx are set, BSx has priority.
pub type BR5_W<'a, REG> = crate::BitWriter<'a, REG, BR5>;
impl<'a, REG> BR5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action on the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BR5::B0x0)
    }
    ///Resets the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR5::B0x1)
    }
}
/**Port x reset I/O y These bits are write-only. A read operation always returns 0x0000. Note: If both BSx and BRx are set, BSx has priority.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR6 {
    ///0: No action on the corresponding ODRx bit
    B0x0 = 0,
    ///1: Resets the corresponding ODRx bit
    B0x1 = 1,
}
impl From<BR6> for bool {
    #[inline(always)]
    fn from(variant: BR6) -> Self {
        variant as u8 != 0
    }
}
///Field `BR6` writer - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000. Note: If both BSx and BRx are set, BSx has priority.
pub type BR6_W<'a, REG> = crate::BitWriter<'a, REG, BR6>;
impl<'a, REG> BR6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action on the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BR6::B0x0)
    }
    ///Resets the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR6::B0x1)
    }
}
/**Port x reset I/O y These bits are write-only. A read operation always returns 0x0000. Note: If both BSx and BRx are set, BSx has priority.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR7 {
    ///0: No action on the corresponding ODRx bit
    B0x0 = 0,
    ///1: Resets the corresponding ODRx bit
    B0x1 = 1,
}
impl From<BR7> for bool {
    #[inline(always)]
    fn from(variant: BR7) -> Self {
        variant as u8 != 0
    }
}
///Field `BR7` writer - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000. Note: If both BSx and BRx are set, BSx has priority.
pub type BR7_W<'a, REG> = crate::BitWriter<'a, REG, BR7>;
impl<'a, REG> BR7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action on the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BR7::B0x0)
    }
    ///Resets the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR7::B0x1)
    }
}
/**Port x reset I/O y These bits are write-only. A read operation always returns 0x0000. Note: If both BSx and BRx are set, BSx has priority.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR8 {
    ///0: No action on the corresponding ODRx bit
    B0x0 = 0,
    ///1: Resets the corresponding ODRx bit
    B0x1 = 1,
}
impl From<BR8> for bool {
    #[inline(always)]
    fn from(variant: BR8) -> Self {
        variant as u8 != 0
    }
}
///Field `BR8` writer - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000. Note: If both BSx and BRx are set, BSx has priority.
pub type BR8_W<'a, REG> = crate::BitWriter<'a, REG, BR8>;
impl<'a, REG> BR8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action on the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BR8::B0x0)
    }
    ///Resets the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR8::B0x1)
    }
}
/**Port x reset I/O y These bits are write-only. A read operation always returns 0x0000. Note: If both BSx and BRx are set, BSx has priority.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR9 {
    ///0: No action on the corresponding ODRx bit
    B0x0 = 0,
    ///1: Resets the corresponding ODRx bit
    B0x1 = 1,
}
impl From<BR9> for bool {
    #[inline(always)]
    fn from(variant: BR9) -> Self {
        variant as u8 != 0
    }
}
///Field `BR9` writer - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000. Note: If both BSx and BRx are set, BSx has priority.
pub type BR9_W<'a, REG> = crate::BitWriter<'a, REG, BR9>;
impl<'a, REG> BR9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action on the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BR9::B0x0)
    }
    ///Resets the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR9::B0x1)
    }
}
/**Port x reset I/O y These bits are write-only. A read operation always returns 0x0000. Note: If both BSx and BRx are set, BSx has priority.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR10 {
    ///0: No action on the corresponding ODRx bit
    B0x0 = 0,
    ///1: Resets the corresponding ODRx bit
    B0x1 = 1,
}
impl From<BR10> for bool {
    #[inline(always)]
    fn from(variant: BR10) -> Self {
        variant as u8 != 0
    }
}
///Field `BR10` writer - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000. Note: If both BSx and BRx are set, BSx has priority.
pub type BR10_W<'a, REG> = crate::BitWriter<'a, REG, BR10>;
impl<'a, REG> BR10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action on the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BR10::B0x0)
    }
    ///Resets the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR10::B0x1)
    }
}
/**Port x reset I/O y These bits are write-only. A read operation always returns 0x0000. Note: If both BSx and BRx are set, BSx has priority.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR11 {
    ///0: No action on the corresponding ODRx bit
    B0x0 = 0,
    ///1: Resets the corresponding ODRx bit
    B0x1 = 1,
}
impl From<BR11> for bool {
    #[inline(always)]
    fn from(variant: BR11) -> Self {
        variant as u8 != 0
    }
}
///Field `BR11` writer - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000. Note: If both BSx and BRx are set, BSx has priority.
pub type BR11_W<'a, REG> = crate::BitWriter<'a, REG, BR11>;
impl<'a, REG> BR11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action on the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BR11::B0x0)
    }
    ///Resets the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR11::B0x1)
    }
}
/**Port x reset I/O y These bits are write-only. A read operation always returns 0x0000. Note: If both BSx and BRx are set, BSx has priority.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR12 {
    ///0: No action on the corresponding ODRx bit
    B0x0 = 0,
    ///1: Resets the corresponding ODRx bit
    B0x1 = 1,
}
impl From<BR12> for bool {
    #[inline(always)]
    fn from(variant: BR12) -> Self {
        variant as u8 != 0
    }
}
///Field `BR12` writer - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000. Note: If both BSx and BRx are set, BSx has priority.
pub type BR12_W<'a, REG> = crate::BitWriter<'a, REG, BR12>;
impl<'a, REG> BR12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action on the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BR12::B0x0)
    }
    ///Resets the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR12::B0x1)
    }
}
/**Port x reset I/O y These bits are write-only. A read operation always returns 0x0000. Note: If both BSx and BRx are set, BSx has priority.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR13 {
    ///0: No action on the corresponding ODRx bit
    B0x0 = 0,
    ///1: Resets the corresponding ODRx bit
    B0x1 = 1,
}
impl From<BR13> for bool {
    #[inline(always)]
    fn from(variant: BR13) -> Self {
        variant as u8 != 0
    }
}
///Field `BR13` writer - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000. Note: If both BSx and BRx are set, BSx has priority.
pub type BR13_W<'a, REG> = crate::BitWriter<'a, REG, BR13>;
impl<'a, REG> BR13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action on the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BR13::B0x0)
    }
    ///Resets the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR13::B0x1)
    }
}
/**Port x reset I/O y These bits are write-only. A read operation always returns 0x0000. Note: If both BSx and BRx are set, BSx has priority.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR14 {
    ///0: No action on the corresponding ODRx bit
    B0x0 = 0,
    ///1: Resets the corresponding ODRx bit
    B0x1 = 1,
}
impl From<BR14> for bool {
    #[inline(always)]
    fn from(variant: BR14) -> Self {
        variant as u8 != 0
    }
}
///Field `BR14` writer - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000. Note: If both BSx and BRx are set, BSx has priority.
pub type BR14_W<'a, REG> = crate::BitWriter<'a, REG, BR14>;
impl<'a, REG> BR14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action on the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BR14::B0x0)
    }
    ///Resets the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR14::B0x1)
    }
}
/**Port x reset I/O y These bits are write-only. A read operation always returns 0x0000. Note: If both BSx and BRx are set, BSx has priority.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR15 {
    ///0: No action on the corresponding ODRx bit
    B0x0 = 0,
    ///1: Resets the corresponding ODRx bit
    B0x1 = 1,
}
impl From<BR15> for bool {
    #[inline(always)]
    fn from(variant: BR15) -> Self {
        variant as u8 != 0
    }
}
///Field `BR15` writer - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000. Note: If both BSx and BRx are set, BSx has priority.
pub type BR15_W<'a, REG> = crate::BitWriter<'a, REG, BR15>;
impl<'a, REG> BR15_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action on the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BR15::B0x0)
    }
    ///Resets the corresponding ODRx bit
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR15::B0x1)
    }
}
impl core::fmt::Debug for crate::generic::Reg<GPIOB_BSRRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Port x set I/O y These bits are write-only. A read operation always returns 0x0000.
    #[inline(always)]
    pub fn bs0(&mut self) -> BS0_W<'_, GPIOB_BSRRrs> {
        BS0_W::new(self, 0)
    }
    ///Bit 1 - Port x set I/O y These bits are write-only. A read operation always returns 0x0000.
    #[inline(always)]
    pub fn bs1(&mut self) -> BS1_W<'_, GPIOB_BSRRrs> {
        BS1_W::new(self, 1)
    }
    ///Bit 2 - Port x set I/O y These bits are write-only. A read operation always returns 0x0000.
    #[inline(always)]
    pub fn bs2(&mut self) -> BS2_W<'_, GPIOB_BSRRrs> {
        BS2_W::new(self, 2)
    }
    ///Bit 3 - Port x set I/O y These bits are write-only. A read operation always returns 0x0000.
    #[inline(always)]
    pub fn bs3(&mut self) -> BS3_W<'_, GPIOB_BSRRrs> {
        BS3_W::new(self, 3)
    }
    ///Bit 4 - Port x set I/O y These bits are write-only. A read operation always returns 0x0000.
    #[inline(always)]
    pub fn bs4(&mut self) -> BS4_W<'_, GPIOB_BSRRrs> {
        BS4_W::new(self, 4)
    }
    ///Bit 5 - Port x set I/O y These bits are write-only. A read operation always returns 0x0000.
    #[inline(always)]
    pub fn bs5(&mut self) -> BS5_W<'_, GPIOB_BSRRrs> {
        BS5_W::new(self, 5)
    }
    ///Bit 6 - Port x set I/O y These bits are write-only. A read operation always returns 0x0000.
    #[inline(always)]
    pub fn bs6(&mut self) -> BS6_W<'_, GPIOB_BSRRrs> {
        BS6_W::new(self, 6)
    }
    ///Bit 7 - Port x set I/O y These bits are write-only. A read operation always returns 0x0000.
    #[inline(always)]
    pub fn bs7(&mut self) -> BS7_W<'_, GPIOB_BSRRrs> {
        BS7_W::new(self, 7)
    }
    ///Bit 8 - Port x set I/O y These bits are write-only. A read operation always returns 0x0000.
    #[inline(always)]
    pub fn bs8(&mut self) -> BS8_W<'_, GPIOB_BSRRrs> {
        BS8_W::new(self, 8)
    }
    ///Bit 9 - Port x set I/O y These bits are write-only. A read operation always returns 0x0000.
    #[inline(always)]
    pub fn bs9(&mut self) -> BS9_W<'_, GPIOB_BSRRrs> {
        BS9_W::new(self, 9)
    }
    ///Bit 10 - Port x set I/O y These bits are write-only. A read operation always returns 0x0000.
    #[inline(always)]
    pub fn bs10(&mut self) -> BS10_W<'_, GPIOB_BSRRrs> {
        BS10_W::new(self, 10)
    }
    ///Bit 11 - Port x set I/O y These bits are write-only. A read operation always returns 0x0000.
    #[inline(always)]
    pub fn bs11(&mut self) -> BS11_W<'_, GPIOB_BSRRrs> {
        BS11_W::new(self, 11)
    }
    ///Bit 12 - Port x set I/O y These bits are write-only. A read operation always returns 0x0000.
    #[inline(always)]
    pub fn bs12(&mut self) -> BS12_W<'_, GPIOB_BSRRrs> {
        BS12_W::new(self, 12)
    }
    ///Bit 13 - Port x set I/O y These bits are write-only. A read operation always returns 0x0000.
    #[inline(always)]
    pub fn bs13(&mut self) -> BS13_W<'_, GPIOB_BSRRrs> {
        BS13_W::new(self, 13)
    }
    ///Bit 14 - Port x set I/O y These bits are write-only. A read operation always returns 0x0000.
    #[inline(always)]
    pub fn bs14(&mut self) -> BS14_W<'_, GPIOB_BSRRrs> {
        BS14_W::new(self, 14)
    }
    ///Bit 15 - Port x set I/O y These bits are write-only. A read operation always returns 0x0000.
    #[inline(always)]
    pub fn bs15(&mut self) -> BS15_W<'_, GPIOB_BSRRrs> {
        BS15_W::new(self, 15)
    }
    ///Bit 16 - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000. Note: If both BSx and BRx are set, BSx has priority.
    #[inline(always)]
    pub fn br0(&mut self) -> BR0_W<'_, GPIOB_BSRRrs> {
        BR0_W::new(self, 16)
    }
    ///Bit 17 - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000. Note: If both BSx and BRx are set, BSx has priority.
    #[inline(always)]
    pub fn br1(&mut self) -> BR1_W<'_, GPIOB_BSRRrs> {
        BR1_W::new(self, 17)
    }
    ///Bit 18 - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000. Note: If both BSx and BRx are set, BSx has priority.
    #[inline(always)]
    pub fn br2(&mut self) -> BR2_W<'_, GPIOB_BSRRrs> {
        BR2_W::new(self, 18)
    }
    ///Bit 19 - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000. Note: If both BSx and BRx are set, BSx has priority.
    #[inline(always)]
    pub fn br3(&mut self) -> BR3_W<'_, GPIOB_BSRRrs> {
        BR3_W::new(self, 19)
    }
    ///Bit 20 - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000. Note: If both BSx and BRx are set, BSx has priority.
    #[inline(always)]
    pub fn br4(&mut self) -> BR4_W<'_, GPIOB_BSRRrs> {
        BR4_W::new(self, 20)
    }
    ///Bit 21 - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000. Note: If both BSx and BRx are set, BSx has priority.
    #[inline(always)]
    pub fn br5(&mut self) -> BR5_W<'_, GPIOB_BSRRrs> {
        BR5_W::new(self, 21)
    }
    ///Bit 22 - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000. Note: If both BSx and BRx are set, BSx has priority.
    #[inline(always)]
    pub fn br6(&mut self) -> BR6_W<'_, GPIOB_BSRRrs> {
        BR6_W::new(self, 22)
    }
    ///Bit 23 - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000. Note: If both BSx and BRx are set, BSx has priority.
    #[inline(always)]
    pub fn br7(&mut self) -> BR7_W<'_, GPIOB_BSRRrs> {
        BR7_W::new(self, 23)
    }
    ///Bit 24 - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000. Note: If both BSx and BRx are set, BSx has priority.
    #[inline(always)]
    pub fn br8(&mut self) -> BR8_W<'_, GPIOB_BSRRrs> {
        BR8_W::new(self, 24)
    }
    ///Bit 25 - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000. Note: If both BSx and BRx are set, BSx has priority.
    #[inline(always)]
    pub fn br9(&mut self) -> BR9_W<'_, GPIOB_BSRRrs> {
        BR9_W::new(self, 25)
    }
    ///Bit 26 - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000. Note: If both BSx and BRx are set, BSx has priority.
    #[inline(always)]
    pub fn br10(&mut self) -> BR10_W<'_, GPIOB_BSRRrs> {
        BR10_W::new(self, 26)
    }
    ///Bit 27 - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000. Note: If both BSx and BRx are set, BSx has priority.
    #[inline(always)]
    pub fn br11(&mut self) -> BR11_W<'_, GPIOB_BSRRrs> {
        BR11_W::new(self, 27)
    }
    ///Bit 28 - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000. Note: If both BSx and BRx are set, BSx has priority.
    #[inline(always)]
    pub fn br12(&mut self) -> BR12_W<'_, GPIOB_BSRRrs> {
        BR12_W::new(self, 28)
    }
    ///Bit 29 - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000. Note: If both BSx and BRx are set, BSx has priority.
    #[inline(always)]
    pub fn br13(&mut self) -> BR13_W<'_, GPIOB_BSRRrs> {
        BR13_W::new(self, 29)
    }
    ///Bit 30 - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000. Note: If both BSx and BRx are set, BSx has priority.
    #[inline(always)]
    pub fn br14(&mut self) -> BR14_W<'_, GPIOB_BSRRrs> {
        BR14_W::new(self, 30)
    }
    ///Bit 31 - Port x reset I/O y These bits are write-only. A read operation always returns 0x0000. Note: If both BSx and BRx are set, BSx has priority.
    #[inline(always)]
    pub fn br15(&mut self) -> BR15_W<'_, GPIOB_BSRRrs> {
        BR15_W::new(self, 31)
    }
}
/**GPIO port bit set/reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_bsrr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#GPIOB:GPIOB_BSRR)*/
pub struct GPIOB_BSRRrs;
impl crate::RegisterSpec for GPIOB_BSRRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`gpiob_bsrr::W`](W) writer structure
impl crate::Writable for GPIOB_BSRRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GPIOB_BSRR to value 0
impl crate::Resettable for GPIOB_BSRRrs {}
