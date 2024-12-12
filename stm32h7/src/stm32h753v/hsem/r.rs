///Register `R%s` reader
pub type R = crate::R<Rrs>;
///Register `R%s` writer
pub type W = crate::W<Rrs>;
///Field `PROCID` reader - Semaphore ProcessID
pub type PROCID_R = crate::FieldReader;
///Field `PROCID` writer - Semaphore ProcessID
pub type PROCID_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `MASTERID` reader - Semaphore MasterID
pub type MASTERID_R = crate::FieldReader;
///Field `MASTERID` writer - Semaphore MasterID
pub type MASTERID_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `LOCK` reader - Lock indication
pub type LOCK_R = crate::BitReader;
///Field `LOCK` writer - Lock indication
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - Semaphore ProcessID
    #[inline(always)]
    pub fn procid(&self) -> PROCID_R {
        PROCID_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Semaphore MasterID
    #[inline(always)]
    pub fn masterid(&self) -> MASTERID_R {
        MASTERID_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bit 31 - Lock indication
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("R")
            .field("procid", &self.procid())
            .field("masterid", &self.masterid())
            .field("lock", &self.lock())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Semaphore ProcessID
    #[inline(always)]
    pub fn procid(&mut self) -> PROCID_W<Rrs> {
        PROCID_W::new(self, 0)
    }
    ///Bits 8:15 - Semaphore MasterID
    #[inline(always)]
    pub fn masterid(&mut self) -> MASTERID_W<Rrs> {
        MASTERID_W::new(self, 8)
    }
    ///Bit 31 - Lock indication
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<Rrs> {
        LOCK_W::new(self, 31)
    }
}
/**HSEM register HSEM_R%s

You can [`read`](crate::Reg::read) this register and get [`r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753.html#HSEM:R[0])*/
pub struct Rrs;
impl crate::RegisterSpec for Rrs {
    type Ux = u32;
}
///`read()` method returns [`r::R`](R) reader structure
impl crate::Readable for Rrs {}
///`write(|w| ..)` method takes [`r::W`](W) writer structure
impl crate::Writable for Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets R%s to value 0
impl crate::Resettable for Rrs {
    const RESET_VALUE: u32 = 0;
}
