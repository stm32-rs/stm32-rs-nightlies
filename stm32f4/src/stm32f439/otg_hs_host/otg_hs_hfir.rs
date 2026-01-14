///Register `OTG_HS_HFIR` reader
pub type R = crate::R<OTG_HS_HFIRrs>;
///Register `OTG_HS_HFIR` writer
pub type W = crate::W<OTG_HS_HFIRrs>;
///Field `FRIVL` reader - Frame interval
pub type FRIVL_R = crate::FieldReader<u16>;
///Field `FRIVL` writer - Frame interval
pub type FRIVL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Frame interval
    #[inline(always)]
    pub fn frivl(&self) -> FRIVL_R {
        FRIVL_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTG_HS_HFIR")
            .field("frivl", &self.frivl())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Frame interval
    #[inline(always)]
    pub fn frivl(&mut self) -> FRIVL_W<'_, OTG_HS_HFIRrs> {
        FRIVL_W::new(self, 0)
    }
}
/**OTG_HS Host frame interval register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hfir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hfir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_HOST:OTG_HS_HFIR)*/
pub struct OTG_HS_HFIRrs;
impl crate::RegisterSpec for OTG_HS_HFIRrs {
    type Ux = u32;
}
///`read()` method returns [`otg_hs_hfir::R`](R) reader structure
impl crate::Readable for OTG_HS_HFIRrs {}
///`write(|w| ..)` method takes [`otg_hs_hfir::W`](W) writer structure
impl crate::Writable for OTG_HS_HFIRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OTG_HS_HFIR to value 0xea60
impl crate::Resettable for OTG_HS_HFIRrs {
    const RESET_VALUE: u32 = 0xea60;
}
