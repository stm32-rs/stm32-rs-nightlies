///Register `DHR12R1` reader
pub type R = crate::R<DHR12R1rs>;
///Register `DHR12R1` writer
pub type W = crate::W<DHR12R1rs>;
///Field `DACC1DHR` reader - DAC channel1 12-bit right-aligned data These bits are written by software. They specify 12-bit data for DAC channel1.
pub type DACC1DHR_R = crate::FieldReader<u16>;
///Field `DACC1DHR` writer - DAC channel1 12-bit right-aligned data These bits are written by software. They specify 12-bit data for DAC channel1.
pub type DACC1DHR_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `DACC1DHRB` reader - DAC channel1 12-bit right-aligned data B These bits are written by software. They specify 12-bit data for DAC channel1 when the DAC operates in Double data mode.
pub type DACC1DHRB_R = crate::FieldReader<u16>;
///Field `DACC1DHRB` writer - DAC channel1 12-bit right-aligned data B These bits are written by software. They specify 12-bit data for DAC channel1 when the DAC operates in Double data mode.
pub type DACC1DHRB_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:11 - DAC channel1 12-bit right-aligned data These bits are written by software. They specify 12-bit data for DAC channel1.
    #[inline(always)]
    pub fn dacc1dhr(&self) -> DACC1DHR_R {
        DACC1DHR_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:27 - DAC channel1 12-bit right-aligned data B These bits are written by software. They specify 12-bit data for DAC channel1 when the DAC operates in Double data mode.
    #[inline(always)]
    pub fn dacc1dhrb(&self) -> DACC1DHRB_R {
        DACC1DHRB_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHR12R1")
            .field("dacc1dhr", &self.dacc1dhr())
            .field("dacc1dhrb", &self.dacc1dhrb())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - DAC channel1 12-bit right-aligned data These bits are written by software. They specify 12-bit data for DAC channel1.
    #[inline(always)]
    pub fn dacc1dhr(&mut self) -> DACC1DHR_W<DHR12R1rs> {
        DACC1DHR_W::new(self, 0)
    }
    ///Bits 16:27 - DAC channel1 12-bit right-aligned data B These bits are written by software. They specify 12-bit data for DAC channel1 when the DAC operates in Double data mode.
    #[inline(always)]
    pub fn dacc1dhrb(&mut self) -> DACC1DHRB_W<DHR12R1rs> {
        DACC1DHRB_W::new(self, 16)
    }
}
/**DAC channel1 12-bit right-aligned data holding register

You can [`read`](crate::Reg::read) this register and get [`dhr12r1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12r1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#DAC:DHR12R1)*/
pub struct DHR12R1rs;
impl crate::RegisterSpec for DHR12R1rs {
    type Ux = u32;
}
///`read()` method returns [`dhr12r1::R`](R) reader structure
impl crate::Readable for DHR12R1rs {}
///`write(|w| ..)` method takes [`dhr12r1::W`](W) writer structure
impl crate::Writable for DHR12R1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHR12R1 to value 0
impl crate::Resettable for DHR12R1rs {}
