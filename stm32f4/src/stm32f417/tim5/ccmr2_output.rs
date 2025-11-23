///Register `CCMR2_Output` reader
pub type R = crate::R<CCMR2_OUTPUTrs>;
///Register `CCMR2_Output` writer
pub type W = crate::W<CCMR2_OUTPUTrs>;
///Field `CC3S` reader - CC3S
pub type CC3S_R = crate::FieldReader;
///Field `CC3S` writer - CC3S
pub type CC3S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OC3FE` reader - OC3FE
pub type OC3FE_R = crate::BitReader;
///Field `OC3FE` writer - OC3FE
pub type OC3FE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC3PE` reader - OC3PE
pub type OC3PE_R = crate::BitReader;
///Field `OC3PE` writer - OC3PE
pub type OC3PE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC3M` reader - OC3M
pub type OC3M_R = crate::FieldReader;
///Field `OC3M` writer - OC3M
pub type OC3M_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `OC3CE` reader - OC3CE
pub type OC3CE_R = crate::BitReader;
///Field `OC3CE` writer - OC3CE
pub type OC3CE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC4S` reader - CC4S
pub type CC4S_R = crate::FieldReader;
///Field `CC4S` writer - CC4S
pub type CC4S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OC4FE` reader - OC4FE
pub type OC4FE_R = crate::BitReader;
///Field `OC4FE` writer - OC4FE
pub type OC4FE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC4PE` reader - OC4PE
pub type OC4PE_R = crate::BitReader;
///Field `OC4PE` writer - OC4PE
pub type OC4PE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC4M` reader - OC4M
pub type OC4M_R = crate::FieldReader;
///Field `OC4M` writer - OC4M
pub type OC4M_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `O24CE` reader - O24CE
pub type O24CE_R = crate::BitReader;
///Field `O24CE` writer - O24CE
pub type O24CE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - CC3S
    #[inline(always)]
    pub fn cc3s(&self) -> CC3S_R {
        CC3S_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - OC3FE
    #[inline(always)]
    pub fn oc3fe(&self) -> OC3FE_R {
        OC3FE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - OC3PE
    #[inline(always)]
    pub fn oc3pe(&self) -> OC3PE_R {
        OC3PE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - OC3M
    #[inline(always)]
    pub fn oc3m(&self) -> OC3M_R {
        OC3M_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - OC3CE
    #[inline(always)]
    pub fn oc3ce(&self) -> OC3CE_R {
        OC3CE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - CC4S
    #[inline(always)]
    pub fn cc4s(&self) -> CC4S_R {
        CC4S_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 10 - OC4FE
    #[inline(always)]
    pub fn oc4fe(&self) -> OC4FE_R {
        OC4FE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - OC4PE
    #[inline(always)]
    pub fn oc4pe(&self) -> OC4PE_R {
        OC4PE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:14 - OC4M
    #[inline(always)]
    pub fn oc4m(&self) -> OC4M_R {
        OC4M_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bit 15 - O24CE
    #[inline(always)]
    pub fn o24ce(&self) -> O24CE_R {
        O24CE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCMR2_Output")
            .field("o24ce", &self.o24ce())
            .field("oc4m", &self.oc4m())
            .field("oc4pe", &self.oc4pe())
            .field("oc4fe", &self.oc4fe())
            .field("cc4s", &self.cc4s())
            .field("oc3ce", &self.oc3ce())
            .field("oc3m", &self.oc3m())
            .field("oc3pe", &self.oc3pe())
            .field("oc3fe", &self.oc3fe())
            .field("cc3s", &self.cc3s())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - CC3S
    #[inline(always)]
    pub fn cc3s(&mut self) -> CC3S_W<'_, CCMR2_OUTPUTrs> {
        CC3S_W::new(self, 0)
    }
    ///Bit 2 - OC3FE
    #[inline(always)]
    pub fn oc3fe(&mut self) -> OC3FE_W<'_, CCMR2_OUTPUTrs> {
        OC3FE_W::new(self, 2)
    }
    ///Bit 3 - OC3PE
    #[inline(always)]
    pub fn oc3pe(&mut self) -> OC3PE_W<'_, CCMR2_OUTPUTrs> {
        OC3PE_W::new(self, 3)
    }
    ///Bits 4:6 - OC3M
    #[inline(always)]
    pub fn oc3m(&mut self) -> OC3M_W<'_, CCMR2_OUTPUTrs> {
        OC3M_W::new(self, 4)
    }
    ///Bit 7 - OC3CE
    #[inline(always)]
    pub fn oc3ce(&mut self) -> OC3CE_W<'_, CCMR2_OUTPUTrs> {
        OC3CE_W::new(self, 7)
    }
    ///Bits 8:9 - CC4S
    #[inline(always)]
    pub fn cc4s(&mut self) -> CC4S_W<'_, CCMR2_OUTPUTrs> {
        CC4S_W::new(self, 8)
    }
    ///Bit 10 - OC4FE
    #[inline(always)]
    pub fn oc4fe(&mut self) -> OC4FE_W<'_, CCMR2_OUTPUTrs> {
        OC4FE_W::new(self, 10)
    }
    ///Bit 11 - OC4PE
    #[inline(always)]
    pub fn oc4pe(&mut self) -> OC4PE_W<'_, CCMR2_OUTPUTrs> {
        OC4PE_W::new(self, 11)
    }
    ///Bits 12:14 - OC4M
    #[inline(always)]
    pub fn oc4m(&mut self) -> OC4M_W<'_, CCMR2_OUTPUTrs> {
        OC4M_W::new(self, 12)
    }
    ///Bit 15 - O24CE
    #[inline(always)]
    pub fn o24ce(&mut self) -> O24CE_W<'_, CCMR2_OUTPUTrs> {
        O24CE_W::new(self, 15)
    }
}
/**capture/compare mode register 2 (output mode)

You can [`read`](crate::Reg::read) this register and get [`ccmr2_output::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr2_output::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#TIM5:CCMR2_Output)*/
pub struct CCMR2_OUTPUTrs;
impl crate::RegisterSpec for CCMR2_OUTPUTrs {
    type Ux = u32;
}
///`read()` method returns [`ccmr2_output::R`](R) reader structure
impl crate::Readable for CCMR2_OUTPUTrs {}
///`write(|w| ..)` method takes [`ccmr2_output::W`](W) writer structure
impl crate::Writable for CCMR2_OUTPUTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCMR2_Output to value 0
impl crate::Resettable for CCMR2_OUTPUTrs {}
