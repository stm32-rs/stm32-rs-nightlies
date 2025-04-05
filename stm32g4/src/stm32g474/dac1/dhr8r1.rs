///Register `DHR8R1` reader
pub type R = crate::R<DHR8R1rs>;
///Register `DHR8R1` writer
pub type W = crate::W<DHR8R1rs>;
///Field `DACC1DHR` reader - DAC channel1 8-bit right-aligned data These bits are written by software which specifies 8-bit data for DAC channel1.
pub type DACC1DHR_R = crate::FieldReader;
///Field `DACC1DHR` writer - DAC channel1 8-bit right-aligned data These bits are written by software which specifies 8-bit data for DAC channel1.
pub type DACC1DHR_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
///Field `DACC1DHRB` reader - DAC channel1 8-bit right-aligned data
pub type DACC1DHRB_R = crate::FieldReader;
///Field `DACC1DHRB` writer - DAC channel1 8-bit right-aligned data
pub type DACC1DHRB_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - DAC channel1 8-bit right-aligned data These bits are written by software which specifies 8-bit data for DAC channel1.
    #[inline(always)]
    pub fn dacc1dhr(&self) -> DACC1DHR_R {
        DACC1DHR_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - DAC channel1 8-bit right-aligned data
    #[inline(always)]
    pub fn dacc1dhrb(&self) -> DACC1DHRB_R {
        DACC1DHRB_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHR8R1")
            .field("dacc1dhr", &self.dacc1dhr())
            .field("dacc1dhrb", &self.dacc1dhrb())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - DAC channel1 8-bit right-aligned data These bits are written by software which specifies 8-bit data for DAC channel1.
    #[inline(always)]
    pub fn dacc1dhr(&mut self) -> DACC1DHR_W<DHR8R1rs> {
        DACC1DHR_W::new(self, 0)
    }
    ///Bits 8:15 - DAC channel1 8-bit right-aligned data
    #[inline(always)]
    pub fn dacc1dhrb(&mut self) -> DACC1DHRB_W<DHR8R1rs> {
        DACC1DHRB_W::new(self, 8)
    }
}
/**DAC channel1 8-bit right aligned data holding register

You can [`read`](crate::Reg::read) this register and get [`dhr8r1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr8r1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#DAC1:DHR8R1)*/
pub struct DHR8R1rs;
impl crate::RegisterSpec for DHR8R1rs {
    type Ux = u32;
}
///`read()` method returns [`dhr8r1::R`](R) reader structure
impl crate::Readable for DHR8R1rs {}
///`write(|w| ..)` method takes [`dhr8r1::W`](W) writer structure
impl crate::Writable for DHR8R1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHR8R1 to value 0
impl crate::Resettable for DHR8R1rs {}
