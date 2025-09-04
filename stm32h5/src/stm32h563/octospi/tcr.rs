///Register `TCR` reader
pub type R = crate::R<TCRrs>;
///Register `TCR` writer
pub type W = crate::W<TCRrs>;
///Field `DCYC` reader - Number of dummy cycles This field defines the duration of the dummy phase. In both SDR and DTR modes, it specifies a number of CLK cycles (0-31). It is recommended to have at least six dummy cycles when using memories with DQS activated.
pub type DCYC_R = crate::FieldReader;
///Field `DCYC` writer - Number of dummy cycles This field defines the duration of the dummy phase. In both SDR and DTR modes, it specifies a number of CLK cycles (0-31). It is recommended to have at least six dummy cycles when using memories with DQS activated.
pub type DCYC_W<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
/**Delay hold quarter cycle

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DHQC {
    ///0: No delay hold
    NoDelay = 0,
    ///1: 1/4 cycle hold
    QuarterCycleHold = 1,
}
impl From<DHQC> for bool {
    #[inline(always)]
    fn from(variant: DHQC) -> Self {
        variant as u8 != 0
    }
}
///Field `DHQC` reader - Delay hold quarter cycle
pub type DHQC_R = crate::BitReader<DHQC>;
impl DHQC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DHQC {
        match self.bits {
            false => DHQC::NoDelay,
            true => DHQC::QuarterCycleHold,
        }
    }
    ///No delay hold
    #[inline(always)]
    pub fn is_no_delay(&self) -> bool {
        *self == DHQC::NoDelay
    }
    ///1/4 cycle hold
    #[inline(always)]
    pub fn is_quarter_cycle_hold(&self) -> bool {
        *self == DHQC::QuarterCycleHold
    }
}
///Field `DHQC` writer - Delay hold quarter cycle
pub type DHQC_W<'a, REG> = crate::BitWriter<'a, REG, DHQC>;
impl<'a, REG> DHQC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No delay hold
    #[inline(always)]
    pub fn no_delay(self) -> &'a mut crate::W<REG> {
        self.variant(DHQC::NoDelay)
    }
    ///1/4 cycle hold
    #[inline(always)]
    pub fn quarter_cycle_hold(self) -> &'a mut crate::W<REG> {
        self.variant(DHQC::QuarterCycleHold)
    }
}
/**Sample shift By default, the OCTOSPI samples data 1/2 of a CLK cycle after the data is driven by the external device. This bit allows the data to be sampled later in order to consider the external signal delays. The software must ensure that SSHIFT = 0 when the data phase is configured in DTR mode (when DDTR = 1.)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSHIFT {
    ///0: No shift
    NoShift = 0,
    ///1: 1/2 cycle shift
    HalfCycleShift = 1,
}
impl From<SSHIFT> for bool {
    #[inline(always)]
    fn from(variant: SSHIFT) -> Self {
        variant as u8 != 0
    }
}
///Field `SSHIFT` reader - Sample shift By default, the OCTOSPI samples data 1/2 of a CLK cycle after the data is driven by the external device. This bit allows the data to be sampled later in order to consider the external signal delays. The software must ensure that SSHIFT = 0 when the data phase is configured in DTR mode (when DDTR = 1.)
pub type SSHIFT_R = crate::BitReader<SSHIFT>;
impl SSHIFT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SSHIFT {
        match self.bits {
            false => SSHIFT::NoShift,
            true => SSHIFT::HalfCycleShift,
        }
    }
    ///No shift
    #[inline(always)]
    pub fn is_no_shift(&self) -> bool {
        *self == SSHIFT::NoShift
    }
    ///1/2 cycle shift
    #[inline(always)]
    pub fn is_half_cycle_shift(&self) -> bool {
        *self == SSHIFT::HalfCycleShift
    }
}
///Field `SSHIFT` writer - Sample shift By default, the OCTOSPI samples data 1/2 of a CLK cycle after the data is driven by the external device. This bit allows the data to be sampled later in order to consider the external signal delays. The software must ensure that SSHIFT = 0 when the data phase is configured in DTR mode (when DDTR = 1.)
pub type SSHIFT_W<'a, REG> = crate::BitWriter<'a, REG, SSHIFT>;
impl<'a, REG> SSHIFT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No shift
    #[inline(always)]
    pub fn no_shift(self) -> &'a mut crate::W<REG> {
        self.variant(SSHIFT::NoShift)
    }
    ///1/2 cycle shift
    #[inline(always)]
    pub fn half_cycle_shift(self) -> &'a mut crate::W<REG> {
        self.variant(SSHIFT::HalfCycleShift)
    }
}
impl R {
    ///Bits 0:4 - Number of dummy cycles This field defines the duration of the dummy phase. In both SDR and DTR modes, it specifies a number of CLK cycles (0-31). It is recommended to have at least six dummy cycles when using memories with DQS activated.
    #[inline(always)]
    pub fn dcyc(&self) -> DCYC_R {
        DCYC_R::new((self.bits & 0x1f) as u8)
    }
    ///Bit 28 - Delay hold quarter cycle
    #[inline(always)]
    pub fn dhqc(&self) -> DHQC_R {
        DHQC_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 30 - Sample shift By default, the OCTOSPI samples data 1/2 of a CLK cycle after the data is driven by the external device. This bit allows the data to be sampled later in order to consider the external signal delays. The software must ensure that SSHIFT = 0 when the data phase is configured in DTR mode (when DDTR = 1.)
    #[inline(always)]
    pub fn sshift(&self) -> SSHIFT_R {
        SSHIFT_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TCR")
            .field("dcyc", &self.dcyc())
            .field("dhqc", &self.dhqc())
            .field("sshift", &self.sshift())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - Number of dummy cycles This field defines the duration of the dummy phase. In both SDR and DTR modes, it specifies a number of CLK cycles (0-31). It is recommended to have at least six dummy cycles when using memories with DQS activated.
    #[inline(always)]
    pub fn dcyc(&mut self) -> DCYC_W<TCRrs> {
        DCYC_W::new(self, 0)
    }
    ///Bit 28 - Delay hold quarter cycle
    #[inline(always)]
    pub fn dhqc(&mut self) -> DHQC_W<TCRrs> {
        DHQC_W::new(self, 28)
    }
    ///Bit 30 - Sample shift By default, the OCTOSPI samples data 1/2 of a CLK cycle after the data is driven by the external device. This bit allows the data to be sampled later in order to consider the external signal delays. The software must ensure that SSHIFT = 0 when the data phase is configured in DTR mode (when DDTR = 1.)
    #[inline(always)]
    pub fn sshift(&mut self) -> SSHIFT_W<TCRrs> {
        SSHIFT_W::new(self, 30)
    }
}
/**OCTOSPI timing configuration register

You can [`read`](crate::Reg::read) this register and get [`tcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#OCTOSPI:TCR)*/
pub struct TCRrs;
impl crate::RegisterSpec for TCRrs {
    type Ux = u32;
}
///`read()` method returns [`tcr::R`](R) reader structure
impl crate::Readable for TCRrs {}
///`write(|w| ..)` method takes [`tcr::W`](W) writer structure
impl crate::Writable for TCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TCR to value 0
impl crate::Resettable for TCRrs {}
