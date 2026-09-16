///Register `INIT1` reader
pub type R = crate::R<INIT1rs>;
///Register `INIT1` writer
pub type W = crate::W<INIT1rs>;
///Field `PRE_OCD_X32` reader - PRE_OCD_X32
pub type PRE_OCD_X32_R = crate::FieldReader;
///Field `PRE_OCD_X32` writer - PRE_OCD_X32
pub type PRE_OCD_X32_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DRAM_RSTN_X1024` reader - DRAM_RSTN_X1024
pub type DRAM_RSTN_X1024_R = crate::FieldReader<u16>;
///Field `DRAM_RSTN_X1024` writer - DRAM_RSTN_X1024
pub type DRAM_RSTN_X1024_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:3 - PRE_OCD_X32
    #[inline(always)]
    pub fn pre_ocd_x32(&self) -> PRE_OCD_X32_R {
        PRE_OCD_X32_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 16:24 - DRAM_RSTN_X1024
    #[inline(always)]
    pub fn dram_rstn_x1024(&self) -> DRAM_RSTN_X1024_R {
        DRAM_RSTN_X1024_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INIT1")
            .field("pre_ocd_x32", &self.pre_ocd_x32())
            .field("dram_rstn_x1024", &self.dram_rstn_x1024())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - PRE_OCD_X32
    #[inline(always)]
    pub fn pre_ocd_x32(&mut self) -> PRE_OCD_X32_W<'_, INIT1rs> {
        PRE_OCD_X32_W::new(self, 0)
    }
    ///Bits 16:24 - DRAM_RSTN_X1024
    #[inline(always)]
    pub fn dram_rstn_x1024(&mut self) -> DRAM_RSTN_X1024_W<'_, INIT1rs> {
        DRAM_RSTN_X1024_W::new(self, 16)
    }
}
/**DDRCTRL SDRAM initialization register 1

You can [`read`](crate::Reg::read) this register and get [`init1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`init1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRCTRL:INIT1)*/
pub struct INIT1rs;
impl crate::RegisterSpec for INIT1rs {
    type Ux = u32;
}
///`read()` method returns [`init1::R`](R) reader structure
impl crate::Readable for INIT1rs {}
///`write(|w| ..)` method takes [`init1::W`](W) writer structure
impl crate::Writable for INIT1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets INIT1 to value 0
impl crate::Resettable for INIT1rs {}
