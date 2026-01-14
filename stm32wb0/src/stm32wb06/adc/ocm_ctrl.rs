///Register `OCM_CTRL` reader
pub type R = crate::R<OCM_CTRLrs>;
///Register `OCM_CTRL` writer
pub type W = crate::W<OCM_CTRLrs>;
///Field `OCM_SRC` reader - select the occasional conversion source
pub type OCM_SRC_R = crate::BitReader;
///Field `OCM_SRC` writer - select the occasional conversion source
pub type OCM_SRC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OCM_ENA` reader - start occasional conversion in analog audio and full modes
pub type OCM_ENA_R = crate::BitReader;
///Field `OCM_ENA` writer - start occasional conversion in analog audio and full modes
pub type OCM_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - select the occasional conversion source
    #[inline(always)]
    pub fn ocm_src(&self) -> OCM_SRC_R {
        OCM_SRC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - start occasional conversion in analog audio and full modes
    #[inline(always)]
    pub fn ocm_ena(&self) -> OCM_ENA_R {
        OCM_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OCM_CTRL")
            .field("ocm_ena", &self.ocm_ena())
            .field("ocm_src", &self.ocm_src())
            .finish()
    }
}
impl W {
    ///Bit 0 - select the occasional conversion source
    #[inline(always)]
    pub fn ocm_src(&mut self) -> OCM_SRC_W<'_, OCM_CTRLrs> {
        OCM_SRC_W::new(self, 0)
    }
    ///Bit 1 - start occasional conversion in analog audio and full modes
    #[inline(always)]
    pub fn ocm_ena(&mut self) -> OCM_ENA_W<'_, OCM_CTRLrs> {
        OCM_ENA_W::new(self, 1)
    }
}
/**Occasionnal mode control register

You can [`read`](crate::Reg::read) this register and get [`ocm_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocm_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#ADC:OCM_CTRL)*/
pub struct OCM_CTRLrs;
impl crate::RegisterSpec for OCM_CTRLrs {
    type Ux = u32;
}
///`read()` method returns [`ocm_ctrl::R`](R) reader structure
impl crate::Readable for OCM_CTRLrs {}
///`write(|w| ..)` method takes [`ocm_ctrl::W`](W) writer structure
impl crate::Writable for OCM_CTRLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OCM_CTRL to value 0
impl crate::Resettable for OCM_CTRLrs {}
