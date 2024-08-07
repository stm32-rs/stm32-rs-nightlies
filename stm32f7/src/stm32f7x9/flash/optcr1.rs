///Register `OPTCR1` reader
pub type R = crate::R<OPTCR1rs>;
///Register `OPTCR1` writer
pub type W = crate::W<OPTCR1rs>;
///Field `BOOT_ADD0` reader - Boot base address when Boot pin =0
pub type BOOT_ADD0_R = crate::FieldReader<u16>;
///Field `BOOT_ADD0` writer - Boot base address when Boot pin =0
pub type BOOT_ADD0_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `BOOT_ADD1` reader - Boot base address when Boot pin =1
pub type BOOT_ADD1_R = crate::FieldReader<u16>;
///Field `BOOT_ADD1` writer - Boot base address when Boot pin =1
pub type BOOT_ADD1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Boot base address when Boot pin =0
    #[inline(always)]
    pub fn boot_add0(&self) -> BOOT_ADD0_R {
        BOOT_ADD0_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Boot base address when Boot pin =1
    #[inline(always)]
    pub fn boot_add1(&self) -> BOOT_ADD1_R {
        BOOT_ADD1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPTCR1")
            .field("boot_add0", &self.boot_add0())
            .field("boot_add1", &self.boot_add1())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Boot base address when Boot pin =0
    #[inline(always)]
    #[must_use]
    pub fn boot_add0(&mut self) -> BOOT_ADD0_W<OPTCR1rs> {
        BOOT_ADD0_W::new(self, 0)
    }
    ///Bits 16:31 - Boot base address when Boot pin =1
    #[inline(always)]
    #[must_use]
    pub fn boot_add1(&mut self) -> BOOT_ADD1_W<OPTCR1rs> {
        BOOT_ADD1_W::new(self, 16)
    }
}
/**Flash option control register 1

You can [`read`](crate::Reg::read) this register and get [`optcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F7x9.html#FLASH:OPTCR1)*/
pub struct OPTCR1rs;
impl crate::RegisterSpec for OPTCR1rs {
    type Ux = u32;
}
///`read()` method returns [`optcr1::R`](R) reader structure
impl crate::Readable for OPTCR1rs {}
///`write(|w| ..)` method takes [`optcr1::W`](W) writer structure
impl crate::Writable for OPTCR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OPTCR1 to value 0x0fff_0000
impl crate::Resettable for OPTCR1rs {
    const RESET_VALUE: u32 = 0x0fff_0000;
}
