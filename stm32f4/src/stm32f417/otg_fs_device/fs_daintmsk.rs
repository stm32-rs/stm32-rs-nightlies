///Register `FS_DAINTMSK` reader
pub type R = crate::R<FS_DAINTMSKrs>;
///Register `FS_DAINTMSK` writer
pub type W = crate::W<FS_DAINTMSKrs>;
///Field `IEPM` reader - IN EP interrupt mask bits
pub type IEPM_R = crate::FieldReader<u16>;
///Field `IEPM` writer - IN EP interrupt mask bits
pub type IEPM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `OEPINT` reader - OUT endpoint interrupt bits
pub type OEPINT_R = crate::FieldReader<u16>;
///Field `OEPINT` writer - OUT endpoint interrupt bits
pub type OEPINT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - IN EP interrupt mask bits
    #[inline(always)]
    pub fn iepm(&self) -> IEPM_R {
        IEPM_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - OUT endpoint interrupt bits
    #[inline(always)]
    pub fn oepint(&self) -> OEPINT_R {
        OEPINT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FS_DAINTMSK")
            .field("iepm", &self.iepm())
            .field("oepint", &self.oepint())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - IN EP interrupt mask bits
    #[inline(always)]
    pub fn iepm(&mut self) -> IEPM_W<'_, FS_DAINTMSKrs> {
        IEPM_W::new(self, 0)
    }
    ///Bits 16:31 - OUT endpoint interrupt bits
    #[inline(always)]
    pub fn oepint(&mut self) -> OEPINT_W<'_, FS_DAINTMSKrs> {
        OEPINT_W::new(self, 16)
    }
}
/**OTG_FS all endpoints interrupt mask register (OTG_FS_DAINTMSK)

You can [`read`](crate::Reg::read) this register and get [`fs_daintmsk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_daintmsk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#OTG_FS_DEVICE:FS_DAINTMSK)*/
pub struct FS_DAINTMSKrs;
impl crate::RegisterSpec for FS_DAINTMSKrs {
    type Ux = u32;
}
///`read()` method returns [`fs_daintmsk::R`](R) reader structure
impl crate::Readable for FS_DAINTMSKrs {}
///`write(|w| ..)` method takes [`fs_daintmsk::W`](W) writer structure
impl crate::Writable for FS_DAINTMSKrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FS_DAINTMSK to value 0
impl crate::Resettable for FS_DAINTMSKrs {}
