///Register `OPAMP1_HSOTR` reader
pub type R = crate::R<OPAMP1_HSOTRrs>;
///Register `OPAMP1_HSOTR` writer
pub type W = crate::W<OPAMP1_HSOTRrs>;
///Field `TRIMHSOFFSETN` reader - High-speed mode trim for NMOS differential pairs
pub type TRIMHSOFFSETN_R = crate::FieldReader;
///Field `TRIMHSOFFSETN` writer - High-speed mode trim for NMOS differential pairs
pub type TRIMHSOFFSETN_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `TRIMHSOFFSETP` reader - High-speed mode trim for PMOS differential pairs
pub type TRIMHSOFFSETP_R = crate::FieldReader;
///Field `TRIMHSOFFSETP` writer - High-speed mode trim for PMOS differential pairs
pub type TRIMHSOFFSETP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - High-speed mode trim for NMOS differential pairs
    #[inline(always)]
    pub fn trimhsoffsetn(&self) -> TRIMHSOFFSETN_R {
        TRIMHSOFFSETN_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 8:12 - High-speed mode trim for PMOS differential pairs
    #[inline(always)]
    pub fn trimhsoffsetp(&self) -> TRIMHSOFFSETP_R {
        TRIMHSOFFSETP_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPAMP1_HSOTR")
            .field("trimhsoffsetn", &self.trimhsoffsetn())
            .field("trimhsoffsetp", &self.trimhsoffsetp())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - High-speed mode trim for NMOS differential pairs
    #[inline(always)]
    pub fn trimhsoffsetn(&mut self) -> TRIMHSOFFSETN_W<'_, OPAMP1_HSOTRrs> {
        TRIMHSOFFSETN_W::new(self, 0)
    }
    ///Bits 8:12 - High-speed mode trim for PMOS differential pairs
    #[inline(always)]
    pub fn trimhsoffsetp(&mut self) -> TRIMHSOFFSETP_W<'_, OPAMP1_HSOTRrs> {
        TRIMHSOFFSETP_W::new(self, 8)
    }
}
/**OPAMP1 trimming register in high-speed mode

You can [`read`](crate::Reg::read) this register and get [`opamp1_hsotr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp1_hsotr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#OPAMP1:OPAMP1_HSOTR)*/
pub struct OPAMP1_HSOTRrs;
impl crate::RegisterSpec for OPAMP1_HSOTRrs {
    type Ux = u32;
}
///`read()` method returns [`opamp1_hsotr::R`](R) reader structure
impl crate::Readable for OPAMP1_HSOTRrs {}
///`write(|w| ..)` method takes [`opamp1_hsotr::W`](W) writer structure
impl crate::Writable for OPAMP1_HSOTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OPAMP1_HSOTR to value 0
impl crate::Resettable for OPAMP1_HSOTRrs {}
