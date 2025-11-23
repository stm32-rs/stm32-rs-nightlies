///Register `SDCR1` reader
pub type R = crate::R<SDCR1rs>;
///Register `SDCR1` writer
pub type W = crate::W<SDCR1rs>;
/**Number of column address bits

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NC {
    ///0: 8 bits
    Bits8 = 0,
    ///1: 9 bits
    Bits9 = 1,
    ///2: 10 bits
    Bits10 = 2,
    ///3: 11 bits
    Bits11 = 3,
}
impl From<NC> for u8 {
    #[inline(always)]
    fn from(variant: NC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NC {
    type Ux = u8;
}
impl crate::IsEnum for NC {}
///Field `NC` reader - Number of column address bits
pub type NC_R = crate::FieldReader<NC>;
impl NC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> NC {
        match self.bits {
            0 => NC::Bits8,
            1 => NC::Bits9,
            2 => NC::Bits10,
            3 => NC::Bits11,
            _ => unreachable!(),
        }
    }
    ///8 bits
    #[inline(always)]
    pub fn is_bits8(&self) -> bool {
        *self == NC::Bits8
    }
    ///9 bits
    #[inline(always)]
    pub fn is_bits9(&self) -> bool {
        *self == NC::Bits9
    }
    ///10 bits
    #[inline(always)]
    pub fn is_bits10(&self) -> bool {
        *self == NC::Bits10
    }
    ///11 bits
    #[inline(always)]
    pub fn is_bits11(&self) -> bool {
        *self == NC::Bits11
    }
}
///Field `NC` writer - Number of column address bits
pub type NC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, NC, crate::Safe>;
impl<'a, REG> NC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///8 bits
    #[inline(always)]
    pub fn bits8(self) -> &'a mut crate::W<REG> {
        self.variant(NC::Bits8)
    }
    ///9 bits
    #[inline(always)]
    pub fn bits9(self) -> &'a mut crate::W<REG> {
        self.variant(NC::Bits9)
    }
    ///10 bits
    #[inline(always)]
    pub fn bits10(self) -> &'a mut crate::W<REG> {
        self.variant(NC::Bits10)
    }
    ///11 bits
    #[inline(always)]
    pub fn bits11(self) -> &'a mut crate::W<REG> {
        self.variant(NC::Bits11)
    }
}
/**Number of row address bits

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NR {
    ///0: 11 bits
    Bits11 = 0,
    ///1: 12 bits
    Bits12 = 1,
    ///2: 13 bits
    Bits13 = 2,
}
impl From<NR> for u8 {
    #[inline(always)]
    fn from(variant: NR) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NR {
    type Ux = u8;
}
impl crate::IsEnum for NR {}
///Field `NR` reader - Number of row address bits
pub type NR_R = crate::FieldReader<NR>;
impl NR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<NR> {
        match self.bits {
            0 => Some(NR::Bits11),
            1 => Some(NR::Bits12),
            2 => Some(NR::Bits13),
            _ => None,
        }
    }
    ///11 bits
    #[inline(always)]
    pub fn is_bits11(&self) -> bool {
        *self == NR::Bits11
    }
    ///12 bits
    #[inline(always)]
    pub fn is_bits12(&self) -> bool {
        *self == NR::Bits12
    }
    ///13 bits
    #[inline(always)]
    pub fn is_bits13(&self) -> bool {
        *self == NR::Bits13
    }
}
///Field `NR` writer - Number of row address bits
pub type NR_W<'a, REG> = crate::FieldWriter<'a, REG, 2, NR>;
impl<'a, REG> NR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///11 bits
    #[inline(always)]
    pub fn bits11(self) -> &'a mut crate::W<REG> {
        self.variant(NR::Bits11)
    }
    ///12 bits
    #[inline(always)]
    pub fn bits12(self) -> &'a mut crate::W<REG> {
        self.variant(NR::Bits12)
    }
    ///13 bits
    #[inline(always)]
    pub fn bits13(self) -> &'a mut crate::W<REG> {
        self.variant(NR::Bits13)
    }
}
/**Memory data bus width

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MWID {
    ///0: Memory data bus width 8 bits
    Bits8 = 0,
    ///1: Memory data bus width 16 bits
    Bits16 = 1,
    ///2: Memory data bus width 32 bits
    Bits32 = 2,
}
impl From<MWID> for u8 {
    #[inline(always)]
    fn from(variant: MWID) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MWID {
    type Ux = u8;
}
impl crate::IsEnum for MWID {}
///Field `MWID` reader - Memory data bus width
pub type MWID_R = crate::FieldReader<MWID>;
impl MWID_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<MWID> {
        match self.bits {
            0 => Some(MWID::Bits8),
            1 => Some(MWID::Bits16),
            2 => Some(MWID::Bits32),
            _ => None,
        }
    }
    ///Memory data bus width 8 bits
    #[inline(always)]
    pub fn is_bits8(&self) -> bool {
        *self == MWID::Bits8
    }
    ///Memory data bus width 16 bits
    #[inline(always)]
    pub fn is_bits16(&self) -> bool {
        *self == MWID::Bits16
    }
    ///Memory data bus width 32 bits
    #[inline(always)]
    pub fn is_bits32(&self) -> bool {
        *self == MWID::Bits32
    }
}
///Field `MWID` writer - Memory data bus width
pub type MWID_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MWID>;
impl<'a, REG> MWID_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Memory data bus width 8 bits
    #[inline(always)]
    pub fn bits8(self) -> &'a mut crate::W<REG> {
        self.variant(MWID::Bits8)
    }
    ///Memory data bus width 16 bits
    #[inline(always)]
    pub fn bits16(self) -> &'a mut crate::W<REG> {
        self.variant(MWID::Bits16)
    }
    ///Memory data bus width 32 bits
    #[inline(always)]
    pub fn bits32(self) -> &'a mut crate::W<REG> {
        self.variant(MWID::Bits32)
    }
}
/**Number of internal banks

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NB {
    ///0: Two internal Banks
    Nb2 = 0,
    ///1: Four internal Banks
    Nb4 = 1,
}
impl From<NB> for bool {
    #[inline(always)]
    fn from(variant: NB) -> Self {
        variant as u8 != 0
    }
}
///Field `NB` reader - Number of internal banks
pub type NB_R = crate::BitReader<NB>;
impl NB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> NB {
        match self.bits {
            false => NB::Nb2,
            true => NB::Nb4,
        }
    }
    ///Two internal Banks
    #[inline(always)]
    pub fn is_nb2(&self) -> bool {
        *self == NB::Nb2
    }
    ///Four internal Banks
    #[inline(always)]
    pub fn is_nb4(&self) -> bool {
        *self == NB::Nb4
    }
}
///Field `NB` writer - Number of internal banks
pub type NB_W<'a, REG> = crate::BitWriter<'a, REG, NB>;
impl<'a, REG> NB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Two internal Banks
    #[inline(always)]
    pub fn nb2(self) -> &'a mut crate::W<REG> {
        self.variant(NB::Nb2)
    }
    ///Four internal Banks
    #[inline(always)]
    pub fn nb4(self) -> &'a mut crate::W<REG> {
        self.variant(NB::Nb4)
    }
}
/**CAS latency

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CAS {
    ///1: 1 cycle
    Clocks1 = 1,
    ///2: 2 cycles
    Clocks2 = 2,
    ///3: 3 cycles
    Clocks3 = 3,
}
impl From<CAS> for u8 {
    #[inline(always)]
    fn from(variant: CAS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CAS {
    type Ux = u8;
}
impl crate::IsEnum for CAS {}
///Field `CAS` reader - CAS latency
pub type CAS_R = crate::FieldReader<CAS>;
impl CAS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CAS> {
        match self.bits {
            1 => Some(CAS::Clocks1),
            2 => Some(CAS::Clocks2),
            3 => Some(CAS::Clocks3),
            _ => None,
        }
    }
    ///1 cycle
    #[inline(always)]
    pub fn is_clocks1(&self) -> bool {
        *self == CAS::Clocks1
    }
    ///2 cycles
    #[inline(always)]
    pub fn is_clocks2(&self) -> bool {
        *self == CAS::Clocks2
    }
    ///3 cycles
    #[inline(always)]
    pub fn is_clocks3(&self) -> bool {
        *self == CAS::Clocks3
    }
}
///Field `CAS` writer - CAS latency
pub type CAS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CAS>;
impl<'a, REG> CAS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1 cycle
    #[inline(always)]
    pub fn clocks1(self) -> &'a mut crate::W<REG> {
        self.variant(CAS::Clocks1)
    }
    ///2 cycles
    #[inline(always)]
    pub fn clocks2(self) -> &'a mut crate::W<REG> {
        self.variant(CAS::Clocks2)
    }
    ///3 cycles
    #[inline(always)]
    pub fn clocks3(self) -> &'a mut crate::W<REG> {
        self.variant(CAS::Clocks3)
    }
}
/**Write protection

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WP {
    ///0: Write accesses allowed
    Disabled = 0,
    ///1: Write accesses ignored
    Enabled = 1,
}
impl From<WP> for bool {
    #[inline(always)]
    fn from(variant: WP) -> Self {
        variant as u8 != 0
    }
}
///Field `WP` reader - Write protection
pub type WP_R = crate::BitReader<WP>;
impl WP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WP {
        match self.bits {
            false => WP::Disabled,
            true => WP::Enabled,
        }
    }
    ///Write accesses allowed
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WP::Disabled
    }
    ///Write accesses ignored
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WP::Enabled
    }
}
///Field `WP` writer - Write protection
pub type WP_W<'a, REG> = crate::BitWriter<'a, REG, WP>;
impl<'a, REG> WP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Write accesses allowed
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WP::Disabled)
    }
    ///Write accesses ignored
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(WP::Enabled)
    }
}
/**SDRAM clock configuration

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SDCLK {
    ///0: SDCLK clock disabled
    Disabled = 0,
    ///2: SDCLK period = 2 x HCLK period
    Div2 = 2,
    ///3: SDCLK period = 3 x HCLK period
    Div3 = 3,
}
impl From<SDCLK> for u8 {
    #[inline(always)]
    fn from(variant: SDCLK) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SDCLK {
    type Ux = u8;
}
impl crate::IsEnum for SDCLK {}
///Field `SDCLK` reader - SDRAM clock configuration
pub type SDCLK_R = crate::FieldReader<SDCLK>;
impl SDCLK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SDCLK> {
        match self.bits {
            0 => Some(SDCLK::Disabled),
            2 => Some(SDCLK::Div2),
            3 => Some(SDCLK::Div3),
            _ => None,
        }
    }
    ///SDCLK clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SDCLK::Disabled
    }
    ///SDCLK period = 2 x HCLK period
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == SDCLK::Div2
    }
    ///SDCLK period = 3 x HCLK period
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == SDCLK::Div3
    }
}
///Field `SDCLK` writer - SDRAM clock configuration
pub type SDCLK_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SDCLK>;
impl<'a, REG> SDCLK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///SDCLK clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SDCLK::Disabled)
    }
    ///SDCLK period = 2 x HCLK period
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(SDCLK::Div2)
    }
    ///SDCLK period = 3 x HCLK period
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(SDCLK::Div3)
    }
}
/**Burst read

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RBURST {
    ///0: Single read requests are not managed as bursts
    Disabled = 0,
    ///1: Single read requests are always managed as bursts
    Enabled = 1,
}
impl From<RBURST> for bool {
    #[inline(always)]
    fn from(variant: RBURST) -> Self {
        variant as u8 != 0
    }
}
///Field `RBURST` reader - Burst read
pub type RBURST_R = crate::BitReader<RBURST>;
impl RBURST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RBURST {
        match self.bits {
            false => RBURST::Disabled,
            true => RBURST::Enabled,
        }
    }
    ///Single read requests are not managed as bursts
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RBURST::Disabled
    }
    ///Single read requests are always managed as bursts
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RBURST::Enabled
    }
}
///Field `RBURST` writer - Burst read
pub type RBURST_W<'a, REG> = crate::BitWriter<'a, REG, RBURST>;
impl<'a, REG> RBURST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Single read requests are not managed as bursts
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RBURST::Disabled)
    }
    ///Single read requests are always managed as bursts
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RBURST::Enabled)
    }
}
/**Read pipe

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RPIPE {
    ///0: No clock cycle delay
    NoDelay = 0,
    ///1: One clock cycle delay
    Clocks1 = 1,
    ///2: Two clock cycles delay
    Clocks2 = 2,
}
impl From<RPIPE> for u8 {
    #[inline(always)]
    fn from(variant: RPIPE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RPIPE {
    type Ux = u8;
}
impl crate::IsEnum for RPIPE {}
///Field `RPIPE` reader - Read pipe
pub type RPIPE_R = crate::FieldReader<RPIPE>;
impl RPIPE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<RPIPE> {
        match self.bits {
            0 => Some(RPIPE::NoDelay),
            1 => Some(RPIPE::Clocks1),
            2 => Some(RPIPE::Clocks2),
            _ => None,
        }
    }
    ///No clock cycle delay
    #[inline(always)]
    pub fn is_no_delay(&self) -> bool {
        *self == RPIPE::NoDelay
    }
    ///One clock cycle delay
    #[inline(always)]
    pub fn is_clocks1(&self) -> bool {
        *self == RPIPE::Clocks1
    }
    ///Two clock cycles delay
    #[inline(always)]
    pub fn is_clocks2(&self) -> bool {
        *self == RPIPE::Clocks2
    }
}
///Field `RPIPE` writer - Read pipe
pub type RPIPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, RPIPE>;
impl<'a, REG> RPIPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No clock cycle delay
    #[inline(always)]
    pub fn no_delay(self) -> &'a mut crate::W<REG> {
        self.variant(RPIPE::NoDelay)
    }
    ///One clock cycle delay
    #[inline(always)]
    pub fn clocks1(self) -> &'a mut crate::W<REG> {
        self.variant(RPIPE::Clocks1)
    }
    ///Two clock cycles delay
    #[inline(always)]
    pub fn clocks2(self) -> &'a mut crate::W<REG> {
        self.variant(RPIPE::Clocks2)
    }
}
impl R {
    ///Bits 0:1 - Number of column address bits
    #[inline(always)]
    pub fn nc(&self) -> NC_R {
        NC_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Number of row address bits
    #[inline(always)]
    pub fn nr(&self) -> NR_R {
        NR_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Memory data bus width
    #[inline(always)]
    pub fn mwid(&self) -> MWID_R {
        MWID_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 6 - Number of internal banks
    #[inline(always)]
    pub fn nb(&self) -> NB_R {
        NB_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 7:8 - CAS latency
    #[inline(always)]
    pub fn cas(&self) -> CAS_R {
        CAS_R::new(((self.bits >> 7) & 3) as u8)
    }
    ///Bit 9 - Write protection
    #[inline(always)]
    pub fn wp(&self) -> WP_R {
        WP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 10:11 - SDRAM clock configuration
    #[inline(always)]
    pub fn sdclk(&self) -> SDCLK_R {
        SDCLK_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bit 12 - Burst read
    #[inline(always)]
    pub fn rburst(&self) -> RBURST_R {
        RBURST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:14 - Read pipe
    #[inline(always)]
    pub fn rpipe(&self) -> RPIPE_R {
        RPIPE_R::new(((self.bits >> 13) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDCR1")
            .field("nc", &self.nc())
            .field("nr", &self.nr())
            .field("mwid", &self.mwid())
            .field("nb", &self.nb())
            .field("cas", &self.cas())
            .field("wp", &self.wp())
            .field("sdclk", &self.sdclk())
            .field("rburst", &self.rburst())
            .field("rpipe", &self.rpipe())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Number of column address bits
    #[inline(always)]
    pub fn nc(&mut self) -> NC_W<'_, SDCR1rs> {
        NC_W::new(self, 0)
    }
    ///Bits 2:3 - Number of row address bits
    #[inline(always)]
    pub fn nr(&mut self) -> NR_W<'_, SDCR1rs> {
        NR_W::new(self, 2)
    }
    ///Bits 4:5 - Memory data bus width
    #[inline(always)]
    pub fn mwid(&mut self) -> MWID_W<'_, SDCR1rs> {
        MWID_W::new(self, 4)
    }
    ///Bit 6 - Number of internal banks
    #[inline(always)]
    pub fn nb(&mut self) -> NB_W<'_, SDCR1rs> {
        NB_W::new(self, 6)
    }
    ///Bits 7:8 - CAS latency
    #[inline(always)]
    pub fn cas(&mut self) -> CAS_W<'_, SDCR1rs> {
        CAS_W::new(self, 7)
    }
    ///Bit 9 - Write protection
    #[inline(always)]
    pub fn wp(&mut self) -> WP_W<'_, SDCR1rs> {
        WP_W::new(self, 9)
    }
    ///Bits 10:11 - SDRAM clock configuration
    #[inline(always)]
    pub fn sdclk(&mut self) -> SDCLK_W<'_, SDCR1rs> {
        SDCLK_W::new(self, 10)
    }
    ///Bit 12 - Burst read
    #[inline(always)]
    pub fn rburst(&mut self) -> RBURST_W<'_, SDCR1rs> {
        RBURST_W::new(self, 12)
    }
    ///Bits 13:14 - Read pipe
    #[inline(always)]
    pub fn rpipe(&mut self) -> RPIPE_W<'_, SDCR1rs> {
        RPIPE_W::new(self, 13)
    }
}
/**SDRAM Control Register 1

You can [`read`](crate::Reg::read) this register and get [`sdcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F767.html#FMC:SDCR1)*/
pub struct SDCR1rs;
impl crate::RegisterSpec for SDCR1rs {
    type Ux = u32;
}
///`read()` method returns [`sdcr1::R`](R) reader structure
impl crate::Readable for SDCR1rs {}
///`write(|w| ..)` method takes [`sdcr1::W`](W) writer structure
impl crate::Writable for SDCR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SDCR1 to value 0x02d0
impl crate::Resettable for SDCR1rs {
    const RESET_VALUE: u32 = 0x02d0;
}
