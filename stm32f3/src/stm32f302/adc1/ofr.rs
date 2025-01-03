///Register `OFR%s` reader
pub type R = crate::R<OFRrs>;
///Register `OFR%s` writer
pub type W = crate::W<OFRrs>;
///Field `OFFSET` reader - OFFSET1
pub type OFFSET_R = crate::FieldReader<u16>;
///Field `OFFSET` writer - OFFSET1
pub type OFFSET_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16, crate::Safe>;
///Field `OFFSET_CH` reader - OFFSET1_CH
pub type OFFSET_CH_R = crate::FieldReader;
///Field `OFFSET_CH` writer - OFFSET1_CH
pub type OFFSET_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
/**OFFSET1_EN

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OFFSET1_EN {
    ///0: Offset disabled
    Disabled = 0,
    ///1: Offset enabled
    Enabled = 1,
}
impl From<OFFSET1_EN> for bool {
    #[inline(always)]
    fn from(variant: OFFSET1_EN) -> Self {
        variant as u8 != 0
    }
}
///Field `OFFSET1_EN` reader - OFFSET1_EN
pub type OFFSET1_EN_R = crate::BitReader<OFFSET1_EN>;
impl OFFSET1_EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OFFSET1_EN {
        match self.bits {
            false => OFFSET1_EN::Disabled,
            true => OFFSET1_EN::Enabled,
        }
    }
    ///Offset disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OFFSET1_EN::Disabled
    }
    ///Offset enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OFFSET1_EN::Enabled
    }
}
///Field `OFFSET1_EN` writer - OFFSET1_EN
pub type OFFSET1_EN_W<'a, REG> = crate::BitWriter<'a, REG, OFFSET1_EN>;
impl<'a, REG> OFFSET1_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Offset disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OFFSET1_EN::Disabled)
    }
    ///Offset enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OFFSET1_EN::Enabled)
    }
}
impl R {
    ///Bits 0:11 - OFFSET1
    #[inline(always)]
    pub fn offset(&self) -> OFFSET_R {
        OFFSET_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 26:30 - OFFSET1_CH
    #[inline(always)]
    pub fn offset_ch(&self) -> OFFSET_CH_R {
        OFFSET_CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    ///Bit 31 - OFFSET1_EN
    #[inline(always)]
    pub fn offset1_en(&self) -> OFFSET1_EN_R {
        OFFSET1_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OFR")
            .field("offset1_en", &self.offset1_en())
            .field("offset_ch", &self.offset_ch())
            .field("offset", &self.offset())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - OFFSET1
    #[inline(always)]
    pub fn offset(&mut self) -> OFFSET_W<OFRrs> {
        OFFSET_W::new(self, 0)
    }
    ///Bits 26:30 - OFFSET1_CH
    #[inline(always)]
    pub fn offset_ch(&mut self) -> OFFSET_CH_W<OFRrs> {
        OFFSET_CH_W::new(self, 26)
    }
    ///Bit 31 - OFFSET1_EN
    #[inline(always)]
    pub fn offset1_en(&mut self) -> OFFSET1_EN_W<OFRrs> {
        OFFSET1_EN_W::new(self, 31)
    }
}
/**offset register %s

You can [`read`](crate::Reg::read) this register and get [`ofr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ofr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F302.html#ADC1:OFR[1])*/
pub struct OFRrs;
impl crate::RegisterSpec for OFRrs {
    type Ux = u32;
}
///`read()` method returns [`ofr::R`](R) reader structure
impl crate::Readable for OFRrs {}
///`write(|w| ..)` method takes [`ofr::W`](W) writer structure
impl crate::Writable for OFRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OFR%s to value 0
impl crate::Resettable for OFRrs {
    const RESET_VALUE: u32 = 0;
}
