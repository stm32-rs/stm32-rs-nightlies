///Register `APB5ENR` reader
pub type R = crate::R<APB5ENRrs>;
///Register `APB5ENR` writer
pub type W = crate::W<APB5ENRrs>;
///Field `LTDCEN` reader - LTDC enable
pub type LTDCEN_R = crate::BitReader;
///Field `LTDCEN` writer - LTDC enable
pub type LTDCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DCMIPPEN` reader - DCMIPP enable
pub type DCMIPPEN_R = crate::BitReader;
///Field `DCMIPPEN` writer - DCMIPP enable
pub type DCMIPPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GFXTIMEN` reader - GFXTIM enable
pub type GFXTIMEN_R = crate::BitReader;
///Field `GFXTIMEN` writer - GFXTIM enable
pub type GFXTIMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VENCEN` reader - VENC enable
pub type VENCEN_R = crate::BitReader;
///Field `VENCEN` writer - VENC enable
pub type VENCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSIEN` reader - CSI enable
pub type CSIEN_R = crate::BitReader;
///Field `CSIEN` writer - CSI enable
pub type CSIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - LTDC enable
    #[inline(always)]
    pub fn ltdcen(&self) -> LTDCEN_R {
        LTDCEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DCMIPP enable
    #[inline(always)]
    pub fn dcmippen(&self) -> DCMIPPEN_R {
        DCMIPPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - GFXTIM enable
    #[inline(always)]
    pub fn gfxtimen(&self) -> GFXTIMEN_R {
        GFXTIMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - VENC enable
    #[inline(always)]
    pub fn vencen(&self) -> VENCEN_R {
        VENCEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CSI enable
    #[inline(always)]
    pub fn csien(&self) -> CSIEN_R {
        CSIEN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB5ENR")
            .field("ltdcen", &self.ltdcen())
            .field("dcmippen", &self.dcmippen())
            .field("gfxtimen", &self.gfxtimen())
            .field("vencen", &self.vencen())
            .field("csien", &self.csien())
            .finish()
    }
}
impl W {
    ///Bit 1 - LTDC enable
    #[inline(always)]
    pub fn ltdcen(&mut self) -> LTDCEN_W<'_, APB5ENRrs> {
        LTDCEN_W::new(self, 1)
    }
    ///Bit 2 - DCMIPP enable
    #[inline(always)]
    pub fn dcmippen(&mut self) -> DCMIPPEN_W<'_, APB5ENRrs> {
        DCMIPPEN_W::new(self, 2)
    }
    ///Bit 4 - GFXTIM enable
    #[inline(always)]
    pub fn gfxtimen(&mut self) -> GFXTIMEN_W<'_, APB5ENRrs> {
        GFXTIMEN_W::new(self, 4)
    }
    ///Bit 5 - VENC enable
    #[inline(always)]
    pub fn vencen(&mut self) -> VENCEN_W<'_, APB5ENRrs> {
        VENCEN_W::new(self, 5)
    }
    ///Bit 6 - CSI enable
    #[inline(always)]
    pub fn csien(&mut self) -> CSIEN_W<'_, APB5ENRrs> {
        CSIEN_W::new(self, 6)
    }
}
/**RCC APB5 enable register

You can [`read`](crate::Reg::read) this register and get [`apb5enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb5enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RCC:APB5ENR)*/
pub struct APB5ENRrs;
impl crate::RegisterSpec for APB5ENRrs {
    type Ux = u32;
}
///`read()` method returns [`apb5enr::R`](R) reader structure
impl crate::Readable for APB5ENRrs {}
///`write(|w| ..)` method takes [`apb5enr::W`](W) writer structure
impl crate::Writable for APB5ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB5ENR to value 0
impl crate::Resettable for APB5ENRrs {}
