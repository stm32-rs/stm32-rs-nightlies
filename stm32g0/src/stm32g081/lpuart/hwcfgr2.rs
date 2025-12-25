///Register `HWCFGR2` reader
pub type R = crate::R<HWCFGR2rs>;
///Register `HWCFGR2` writer
pub type W = crate::W<HWCFGR2rs>;
///Field `CFG1` reader - LUART hardware configuration 1
pub type CFG1_R = crate::FieldReader;
///Field `CFG1` writer - LUART hardware configuration 1
pub type CFG1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `CFG2` reader - LUART hardware configuration 2
pub type CFG2_R = crate::FieldReader;
///Field `CFG2` writer - LUART hardware configuration 2
pub type CFG2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - LUART hardware configuration 1
    #[inline(always)]
    pub fn cfg1(&self) -> CFG1_R {
        CFG1_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - LUART hardware configuration 2
    #[inline(always)]
    pub fn cfg2(&self) -> CFG2_R {
        CFG2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWCFGR2")
            .field("cfg1", &self.cfg1())
            .field("cfg2", &self.cfg2())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - LUART hardware configuration 1
    #[inline(always)]
    pub fn cfg1(&mut self) -> CFG1_W<'_, HWCFGR2rs> {
        CFG1_W::new(self, 0)
    }
    ///Bits 4:7 - LUART hardware configuration 2
    #[inline(always)]
    pub fn cfg2(&mut self) -> CFG2_W<'_, HWCFGR2rs> {
        CFG2_W::new(self, 4)
    }
}
/**LPUART Hardware Configuration register 2

You can [`read`](crate::Reg::read) this register and get [`hwcfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwcfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G081.html#LPUART:HWCFGR2)*/
pub struct HWCFGR2rs;
impl crate::RegisterSpec for HWCFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`hwcfgr2::R`](R) reader structure
impl crate::Readable for HWCFGR2rs {}
///`write(|w| ..)` method takes [`hwcfgr2::W`](W) writer structure
impl crate::Writable for HWCFGR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HWCFGR2 to value 0x13
impl crate::Resettable for HWCFGR2rs {
    const RESET_VALUE: u32 = 0x13;
}
