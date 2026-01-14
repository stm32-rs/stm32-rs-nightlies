///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `COREID` reader - CoreID of semaphore to be cleared
pub type COREID_R = crate::FieldReader;
///Field `COREID` writer - CoreID of semaphore to be cleared
pub type COREID_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
///Field `KEY` reader - Semaphore clear Key
pub type KEY_R = crate::FieldReader<u16>;
///Field `KEY` writer - Semaphore clear Key
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    ///Bits 8:11 - CoreID of semaphore to be cleared
    #[inline(always)]
    pub fn coreid(&self) -> COREID_R {
        COREID_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:31 - Semaphore clear Key
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("key", &self.key())
            .field("coreid", &self.coreid())
            .finish()
    }
}
impl W {
    ///Bits 8:11 - CoreID of semaphore to be cleared
    #[inline(always)]
    pub fn coreid(&mut self) -> COREID_W<'_, CRrs> {
        COREID_W::new(self, 8)
    }
    ///Bits 16:31 - Semaphore clear Key
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W<'_, CRrs> {
        KEY_W::new(self, 16)
    }
}
/**Semaphore Clear register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#HSEM:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
