///Register `DAINTMSK` reader
pub type R = crate::R<DAINTMSKrs>;
///Register `DAINTMSK` writer
pub type W = crate::W<DAINTMSKrs>;
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
        f.debug_struct("DAINTMSK")
            .field("iepm", &self.iepm())
            .field("oepint", &self.oepint())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - IN EP interrupt mask bits
    #[inline(always)]
    pub fn iepm(&mut self) -> IEPM_W<DAINTMSKrs> {
        IEPM_W::new(self, 0)
    }
    ///Bits 16:31 - OUT endpoint interrupt bits
    #[inline(always)]
    pub fn oepint(&mut self) -> OEPINT_W<DAINTMSKrs> {
        OEPINT_W::new(self, 16)
    }
}
/**OTG_FS all endpoints interrupt mask register (OTG_FS_DAINTMSK)

You can [`read`](crate::Reg::read) this register and get [`daintmsk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daintmsk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_DEVICE:DAINTMSK)*/
pub struct DAINTMSKrs;
impl crate::RegisterSpec for DAINTMSKrs {
    type Ux = u32;
}
///`read()` method returns [`daintmsk::R`](R) reader structure
impl crate::Readable for DAINTMSKrs {}
///`write(|w| ..)` method takes [`daintmsk::W`](W) writer structure
impl crate::Writable for DAINTMSKrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DAINTMSK to value 0
impl crate::Resettable for DAINTMSKrs {
    const RESET_VALUE: u32 = 0;
}