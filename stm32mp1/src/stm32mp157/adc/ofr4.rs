///Register `OFR4` reader
pub type R = crate::R<OFR4rs>;
///Register `OFR4` writer
pub type W = crate::W<OFR4rs>;
///Field `OFFSET4` reader - OFFSET4
pub type OFFSET4_R = crate::FieldReader<u32>;
///Field `OFFSET4` writer - OFFSET4
pub type OFFSET4_W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
///Field `OFFSET4_CH` reader - OFFSET4_CH
pub type OFFSET4_CH_R = crate::FieldReader;
///Field `OFFSET4_CH` writer - OFFSET4_CH
pub type OFFSET4_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `SSATE` reader - SSATE
pub type SSATE_R = crate::BitReader;
///Field `SSATE` writer - SSATE
pub type SSATE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:25 - OFFSET4
    #[inline(always)]
    pub fn offset4(&self) -> OFFSET4_R {
        OFFSET4_R::new(self.bits & 0x03ff_ffff)
    }
    ///Bits 26:30 - OFFSET4_CH
    #[inline(always)]
    pub fn offset4_ch(&self) -> OFFSET4_CH_R {
        OFFSET4_CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    ///Bit 31 - SSATE
    #[inline(always)]
    pub fn ssate(&self) -> SSATE_R {
        SSATE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OFR4")
            .field("offset4", &self.offset4())
            .field("offset4_ch", &self.offset4_ch())
            .field("ssate", &self.ssate())
            .finish()
    }
}
impl W {
    ///Bits 0:25 - OFFSET4
    #[inline(always)]
    pub fn offset4(&mut self) -> OFFSET4_W<'_, OFR4rs> {
        OFFSET4_W::new(self, 0)
    }
    ///Bits 26:30 - OFFSET4_CH
    #[inline(always)]
    pub fn offset4_ch(&mut self) -> OFFSET4_CH_W<'_, OFR4rs> {
        OFFSET4_CH_W::new(self, 26)
    }
    ///Bit 31 - SSATE
    #[inline(always)]
    pub fn ssate(&mut self) -> SSATE_W<'_, OFR4rs> {
        SSATE_W::new(self, 31)
    }
}
/**ADC offset register

You can [`read`](crate::Reg::read) this register and get [`ofr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ofr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ADC:OFR4)*/
pub struct OFR4rs;
impl crate::RegisterSpec for OFR4rs {
    type Ux = u32;
}
///`read()` method returns [`ofr4::R`](R) reader structure
impl crate::Readable for OFR4rs {}
///`write(|w| ..)` method takes [`ofr4::W`](W) writer structure
impl crate::Writable for OFR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OFR4 to value 0
impl crate::Resettable for OFR4rs {}
