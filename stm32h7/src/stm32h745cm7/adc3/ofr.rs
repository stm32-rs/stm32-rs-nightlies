///Register `OFR%s` reader
pub type R = crate::R<OFRrs>;
///Register `OFR%s` writer
pub type W = crate::W<OFRrs>;
///Field `OFFSET` reader - Data offset X for the channel programmed into bits OFFSET_CH
pub type OFFSET_R = crate::FieldReader<u32>;
///Field `OFFSET` writer - Data offset X for the channel programmed into bits OFFSET_CH
pub type OFFSET_W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32, crate::Safe>;
///Field `OFFSET_CH` reader - Channel selection for the data offset X
pub type OFFSET_CH_R = crate::FieldReader;
///Field `OFFSET_CH` writer - Channel selection for the data offset X
pub type OFFSET_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
/**Signed saturation enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSATE {
    ///0: Offset is subtracted maintaining data integrity and extending result size (9-bit and 17-bit signed format)
    Disabled = 0,
    ///1: Offset is subtracted and result is saturated to maintain result size
    Enabled = 1,
}
impl From<SSATE> for bool {
    #[inline(always)]
    fn from(variant: SSATE) -> Self {
        variant as u8 != 0
    }
}
///Field `SSATE` reader - Signed saturation enable
pub type SSATE_R = crate::BitReader<SSATE>;
impl SSATE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SSATE {
        match self.bits {
            false => SSATE::Disabled,
            true => SSATE::Enabled,
        }
    }
    ///Offset is subtracted maintaining data integrity and extending result size (9-bit and 17-bit signed format)
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SSATE::Disabled
    }
    ///Offset is subtracted and result is saturated to maintain result size
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SSATE::Enabled
    }
}
///Field `SSATE` writer - Signed saturation enable
pub type SSATE_W<'a, REG> = crate::BitWriter<'a, REG, SSATE>;
impl<'a, REG> SSATE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Offset is subtracted maintaining data integrity and extending result size (9-bit and 17-bit signed format)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SSATE::Disabled)
    }
    ///Offset is subtracted and result is saturated to maintain result size
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SSATE::Enabled)
    }
}
impl R {
    ///Bits 0:25 - Data offset X for the channel programmed into bits OFFSET_CH
    #[inline(always)]
    pub fn offset(&self) -> OFFSET_R {
        OFFSET_R::new(self.bits & 0x03ff_ffff)
    }
    ///Bits 26:30 - Channel selection for the data offset X
    #[inline(always)]
    pub fn offset_ch(&self) -> OFFSET_CH_R {
        OFFSET_CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    ///Bit 31 - Signed saturation enable
    #[inline(always)]
    pub fn ssate(&self) -> SSATE_R {
        SSATE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OFR")
            .field("ssate", &self.ssate())
            .field("offset_ch", &self.offset_ch())
            .field("offset", &self.offset())
            .finish()
    }
}
impl W {
    ///Bits 0:25 - Data offset X for the channel programmed into bits OFFSET_CH
    #[inline(always)]
    pub fn offset(&mut self) -> OFFSET_W<'_, OFRrs> {
        OFFSET_W::new(self, 0)
    }
    ///Bits 26:30 - Channel selection for the data offset X
    #[inline(always)]
    pub fn offset_ch(&mut self) -> OFFSET_CH_W<'_, OFRrs> {
        OFFSET_CH_W::new(self, 26)
    }
    ///Bit 31 - Signed saturation enable
    #[inline(always)]
    pub fn ssate(&mut self) -> SSATE_W<'_, OFRrs> {
        SSATE_W::new(self, 31)
    }
}
/**ADC offset number %s register

You can [`read`](crate::Reg::read) this register and get [`ofr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ofr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM7.html#ADC3:OFR[1])*/
pub struct OFRrs;
impl crate::RegisterSpec for OFRrs {
    type Ux = u32;
}
///`read()` method returns [`ofr::R`](R) reader structure
impl crate::Readable for OFRrs {}
///`write(|w| ..)` method takes [`ofr::W`](W) writer structure
impl crate::Writable for OFRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OFR%s to value 0
impl crate::Resettable for OFRrs {}
