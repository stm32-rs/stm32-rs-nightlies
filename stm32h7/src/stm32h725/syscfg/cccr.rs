///Register `CCCR` reader
pub type R = crate::R<CCCRrs>;
///Register `CCCR` writer
pub type W = crate::W<CCCRrs>;
///Field `NCC` reader - NMOS compensation code
pub type NCC_R = crate::FieldReader;
///Field `NCC` writer - NMOS compensation code
pub type NCC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PCC` reader - PMOS compensation code
pub type PCC_R = crate::FieldReader;
///Field `PCC` writer - PMOS compensation code
pub type PCC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - NMOS compensation code
    #[inline(always)]
    pub fn ncc(&self) -> NCC_R {
        NCC_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - PMOS compensation code
    #[inline(always)]
    pub fn pcc(&self) -> PCC_R {
        PCC_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCCR")
            .field("ncc", &self.ncc())
            .field("pcc", &self.pcc())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - NMOS compensation code
    #[inline(always)]
    pub fn ncc(&mut self) -> NCC_W<'_, CCCRrs> {
        NCC_W::new(self, 0)
    }
    ///Bits 4:7 - PMOS compensation code
    #[inline(always)]
    pub fn pcc(&mut self) -> PCC_W<'_, CCCRrs> {
        PCC_W::new(self, 4)
    }
}
/**SYSCFG compensation cell code register

You can [`read`](crate::Reg::read) this register and get [`cccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H725.html#SYSCFG:CCCR)*/
pub struct CCCRrs;
impl crate::RegisterSpec for CCCRrs {
    type Ux = u32;
}
///`read()` method returns [`cccr::R`](R) reader structure
impl crate::Readable for CCCRrs {}
///`write(|w| ..)` method takes [`cccr::W`](W) writer structure
impl crate::Writable for CCCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCCR to value 0
impl crate::Resettable for CCCRrs {}
