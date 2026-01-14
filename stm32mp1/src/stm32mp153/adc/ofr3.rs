///Register `OFR3` reader
pub type R = crate::R<OFR3rs>;
///Register `OFR3` writer
pub type W = crate::W<OFR3rs>;
///Field `OFFSET3` reader - OFFSET3
pub type OFFSET3_R = crate::FieldReader<u32>;
///Field `OFFSET3` writer - OFFSET3
pub type OFFSET3_W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
///Field `OFFSET3_CH` reader - OFFSET3_CH
pub type OFFSET3_CH_R = crate::FieldReader;
///Field `OFFSET3_CH` writer - OFFSET3_CH
pub type OFFSET3_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `SSATE` reader - SSATE
pub type SSATE_R = crate::BitReader;
///Field `SSATE` writer - SSATE
pub type SSATE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:25 - OFFSET3
    #[inline(always)]
    pub fn offset3(&self) -> OFFSET3_R {
        OFFSET3_R::new(self.bits & 0x03ff_ffff)
    }
    ///Bits 26:30 - OFFSET3_CH
    #[inline(always)]
    pub fn offset3_ch(&self) -> OFFSET3_CH_R {
        OFFSET3_CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    ///Bit 31 - SSATE
    #[inline(always)]
    pub fn ssate(&self) -> SSATE_R {
        SSATE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OFR3")
            .field("offset3", &self.offset3())
            .field("offset3_ch", &self.offset3_ch())
            .field("ssate", &self.ssate())
            .finish()
    }
}
impl W {
    ///Bits 0:25 - OFFSET3
    #[inline(always)]
    pub fn offset3(&mut self) -> OFFSET3_W<'_, OFR3rs> {
        OFFSET3_W::new(self, 0)
    }
    ///Bits 26:30 - OFFSET3_CH
    #[inline(always)]
    pub fn offset3_ch(&mut self) -> OFFSET3_CH_W<'_, OFR3rs> {
        OFFSET3_CH_W::new(self, 26)
    }
    ///Bit 31 - SSATE
    #[inline(always)]
    pub fn ssate(&mut self) -> SSATE_W<'_, OFR3rs> {
        SSATE_W::new(self, 31)
    }
}
/**ADC offset register

You can [`read`](crate::Reg::read) this register and get [`ofr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ofr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ADC:OFR3)*/
pub struct OFR3rs;
impl crate::RegisterSpec for OFR3rs {
    type Ux = u32;
}
///`read()` method returns [`ofr3::R`](R) reader structure
impl crate::Readable for OFR3rs {}
///`write(|w| ..)` method takes [`ofr3::W`](W) writer structure
impl crate::Writable for OFR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OFR3 to value 0
impl crate::Resettable for OFR3rs {}
