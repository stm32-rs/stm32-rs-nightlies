///Register `INIT3` reader
pub type R = crate::R<INIT3rs>;
///Register `INIT3` writer
pub type W = crate::W<INIT3rs>;
///Field `EMR` reader - EMR
pub type EMR_R = crate::FieldReader<u16>;
///Field `EMR` writer - EMR
pub type EMR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `MR` reader - MR
pub type MR_R = crate::FieldReader<u16>;
///Field `MR` writer - MR
pub type MR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - EMR
    #[inline(always)]
    pub fn emr(&self) -> EMR_R {
        EMR_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - MR
    #[inline(always)]
    pub fn mr(&self) -> MR_R {
        MR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INIT3")
            .field("emr", &self.emr())
            .field("mr", &self.mr())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - EMR
    #[inline(always)]
    pub fn emr(&mut self) -> EMR_W<'_, INIT3rs> {
        EMR_W::new(self, 0)
    }
    ///Bits 16:31 - MR
    #[inline(always)]
    pub fn mr(&mut self) -> MR_W<'_, INIT3rs> {
        MR_W::new(self, 16)
    }
}
/**DDRCTRL SDRAM initialization register 3

You can [`read`](crate::Reg::read) this register and get [`init3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`init3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRCTRL:INIT3)*/
pub struct INIT3rs;
impl crate::RegisterSpec for INIT3rs {
    type Ux = u32;
}
///`read()` method returns [`init3::R`](R) reader structure
impl crate::Readable for INIT3rs {}
///`write(|w| ..)` method takes [`init3::W`](W) writer structure
impl crate::Writable for INIT3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets INIT3 to value 0x0510
impl crate::Resettable for INIT3rs {
    const RESET_VALUE: u32 = 0x0510;
}
