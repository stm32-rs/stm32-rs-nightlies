///Register `CCCR` reader
pub type R = crate::R<CCCRrs>;
///Register `CCCR` writer
pub type W = crate::W<CCCRrs>;
///Field `NCC1` reader - NMOS compensation code of the I/Os supplied by VsubDD/sub These bits are written by software to define an I/Os compensation cell code for NMOS transistors. This code is applied to the I/Os compensation cell when the CS1 bit of the SYSCFG_CCCSR is set.
pub type NCC1_R = crate::FieldReader;
///Field `NCC1` writer - NMOS compensation code of the I/Os supplied by VsubDD/sub These bits are written by software to define an I/Os compensation cell code for NMOS transistors. This code is applied to the I/Os compensation cell when the CS1 bit of the SYSCFG_CCCSR is set.
pub type NCC1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PCC1` reader - PMOS compensation code of the I/Os supplied by VsubDD/sub These bits are written by software to define an I/Os compensation cell code for PMOS transistors. This code is applied to the I/Os compensation cell when the CS1 bit of the SYSCFG_CCCSR is set.
pub type PCC1_R = crate::FieldReader;
///Field `PCC1` writer - PMOS compensation code of the I/Os supplied by VsubDD/sub These bits are written by software to define an I/Os compensation cell code for PMOS transistors. This code is applied to the I/Os compensation cell when the CS1 bit of the SYSCFG_CCCSR is set.
pub type PCC1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - NMOS compensation code of the I/Os supplied by VsubDD/sub These bits are written by software to define an I/Os compensation cell code for NMOS transistors. This code is applied to the I/Os compensation cell when the CS1 bit of the SYSCFG_CCCSR is set.
    #[inline(always)]
    pub fn ncc1(&self) -> NCC1_R {
        NCC1_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - PMOS compensation code of the I/Os supplied by VsubDD/sub These bits are written by software to define an I/Os compensation cell code for PMOS transistors. This code is applied to the I/Os compensation cell when the CS1 bit of the SYSCFG_CCCSR is set.
    #[inline(always)]
    pub fn pcc1(&self) -> PCC1_R {
        PCC1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCCR")
            .field("ncc1", &self.ncc1())
            .field("pcc1", &self.pcc1())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - NMOS compensation code of the I/Os supplied by VsubDD/sub These bits are written by software to define an I/Os compensation cell code for NMOS transistors. This code is applied to the I/Os compensation cell when the CS1 bit of the SYSCFG_CCCSR is set.
    #[inline(always)]
    pub fn ncc1(&mut self) -> NCC1_W<'_, CCCRrs> {
        NCC1_W::new(self, 0)
    }
    ///Bits 4:7 - PMOS compensation code of the I/Os supplied by VsubDD/sub These bits are written by software to define an I/Os compensation cell code for PMOS transistors. This code is applied to the I/Os compensation cell when the CS1 bit of the SYSCFG_CCCSR is set.
    #[inline(always)]
    pub fn pcc1(&mut self) -> PCC1_W<'_, CCCRrs> {
        PCC1_W::new(self, 4)
    }
}
/**SYSCFG compensation cell code register

You can [`read`](crate::Reg::read) this register and get [`cccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#SYSCFG:CCCR)*/
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
///`reset()` method sets CCCR to value 0x78
impl crate::Resettable for CCCRrs {
    const RESET_VALUE: u32 = 0x78;
}
