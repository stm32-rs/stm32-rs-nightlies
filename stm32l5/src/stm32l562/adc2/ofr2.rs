///Register `OFR2` reader
pub type R = crate::R<OFR2rs>;
///Register `OFR2` writer
pub type W = crate::W<OFR2rs>;
///Field `OFFSET2` reader - OFFSET2
pub type OFFSET2_R = crate::FieldReader<u16>;
///Field `OFFSET2` writer - OFFSET2
pub type OFFSET2_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `OFFSET2_CH` reader - OFFSET2_CH
pub type OFFSET2_CH_R = crate::FieldReader;
///Field `OFFSET2_CH` writer - OFFSET2_CH
pub type OFFSET2_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `OFFSET2_EN` reader - OFFSET2_EN
pub type OFFSET2_EN_R = crate::BitReader;
///Field `OFFSET2_EN` writer - OFFSET2_EN
pub type OFFSET2_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:11 - OFFSET2
    #[inline(always)]
    pub fn offset2(&self) -> OFFSET2_R {
        OFFSET2_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 26:30 - OFFSET2_CH
    #[inline(always)]
    pub fn offset2_ch(&self) -> OFFSET2_CH_R {
        OFFSET2_CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    ///Bit 31 - OFFSET2_EN
    #[inline(always)]
    pub fn offset2_en(&self) -> OFFSET2_EN_R {
        OFFSET2_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OFR2")
            .field("offset2_en", &self.offset2_en())
            .field("offset2_ch", &self.offset2_ch())
            .field("offset2", &self.offset2())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - OFFSET2
    #[inline(always)]
    #[must_use]
    pub fn offset2(&mut self) -> OFFSET2_W<OFR2rs> {
        OFFSET2_W::new(self, 0)
    }
    ///Bits 26:30 - OFFSET2_CH
    #[inline(always)]
    #[must_use]
    pub fn offset2_ch(&mut self) -> OFFSET2_CH_W<OFR2rs> {
        OFFSET2_CH_W::new(self, 26)
    }
    ///Bit 31 - OFFSET2_EN
    #[inline(always)]
    #[must_use]
    pub fn offset2_en(&mut self) -> OFFSET2_EN_W<OFR2rs> {
        OFFSET2_EN_W::new(self, 31)
    }
}
/**offset register 2

You can [`read`](crate::Reg::read) this register and get [`ofr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ofr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#ADC2:OFR2)*/
pub struct OFR2rs;
impl crate::RegisterSpec for OFR2rs {
    type Ux = u32;
}
///`read()` method returns [`ofr2::R`](R) reader structure
impl crate::Readable for OFR2rs {}
///`write(|w| ..)` method takes [`ofr2::W`](W) writer structure
impl crate::Writable for OFR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OFR2 to value 0
impl crate::Resettable for OFR2rs {
    const RESET_VALUE: u32 = 0;
}
