///Register `BOOT_PRGR` reader
pub type R = crate::R<BOOT_PRGRrs>;
///Register `BOOT_PRGR` writer
pub type W = crate::W<BOOT_PRGRrs>;
///Field `BOOT_ADD0` reader - Boot address 0
pub type BOOT_ADD0_R = crate::FieldReader<u16>;
///Field `BOOT_ADD0` writer - Boot address 0
pub type BOOT_ADD0_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `BOOT_ADD1` reader - Boot address 1
pub type BOOT_ADD1_R = crate::FieldReader<u16>;
///Field `BOOT_ADD1` writer - Boot address 1
pub type BOOT_ADD1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Boot address 0
    #[inline(always)]
    pub fn boot_add0(&self) -> BOOT_ADD0_R {
        BOOT_ADD0_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Boot address 1
    #[inline(always)]
    pub fn boot_add1(&self) -> BOOT_ADD1_R {
        BOOT_ADD1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BOOT_PRGR")
            .field("boot_add0", &self.boot_add0())
            .field("boot_add1", &self.boot_add1())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Boot address 0
    #[inline(always)]
    pub fn boot_add0(&mut self) -> BOOT_ADD0_W<'_, BOOT_PRGRrs> {
        BOOT_ADD0_W::new(self, 0)
    }
    ///Bits 16:31 - Boot address 1
    #[inline(always)]
    pub fn boot_add1(&mut self) -> BOOT_ADD1_W<'_, BOOT_PRGRrs> {
        BOOT_ADD1_W::new(self, 16)
    }
}
/**FLASH register with boot address

You can [`read`](crate::Reg::read) this register and get [`boot_prgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`boot_prgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753V.html#FLASH:BOOT_PRGR)*/
pub struct BOOT_PRGRrs;
impl crate::RegisterSpec for BOOT_PRGRrs {
    type Ux = u32;
}
///`read()` method returns [`boot_prgr::R`](R) reader structure
impl crate::Readable for BOOT_PRGRrs {}
///`write(|w| ..)` method takes [`boot_prgr::W`](W) writer structure
impl crate::Writable for BOOT_PRGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BOOT_PRGR to value 0
impl crate::Resettable for BOOT_PRGRrs {}
