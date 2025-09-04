///Register `R31` reader
pub type R = crate::R<R31rs>;
///Register `R31` writer
pub type W = crate::W<R31rs>;
///Field `PROCID` reader - PROCID
pub type PROCID_R = crate::FieldReader;
///Field `PROCID` writer - PROCID
pub type PROCID_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `COREID` reader - COREID
pub type COREID_R = crate::FieldReader;
///Field `COREID` writer - COREID
pub type COREID_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `LOCK` reader - LOCK
pub type LOCK_R = crate::BitReader;
///Field `LOCK` writer - LOCK
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - PROCID
    #[inline(always)]
    pub fn procid(&self) -> PROCID_R {
        PROCID_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - COREID
    #[inline(always)]
    pub fn coreid(&self) -> COREID_R {
        COREID_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 31 - LOCK
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("R31")
            .field("procid", &self.procid())
            .field("coreid", &self.coreid())
            .field("lock", &self.lock())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - PROCID
    #[inline(always)]
    pub fn procid(&mut self) -> PROCID_W<R31rs> {
        PROCID_W::new(self, 0)
    }
    ///Bits 8:11 - COREID
    #[inline(always)]
    pub fn coreid(&mut self) -> COREID_W<R31rs> {
        COREID_W::new(self, 8)
    }
    ///Bit 31 - LOCK
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<R31rs> {
        LOCK_W::new(self, 31)
    }
}
/**The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`r31::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r31::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:R31)*/
pub struct R31rs;
impl crate::RegisterSpec for R31rs {
    type Ux = u32;
}
///`read()` method returns [`r31::R`](R) reader structure
impl crate::Readable for R31rs {}
///`write(|w| ..)` method takes [`r31::W`](W) writer structure
impl crate::Writable for R31rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets R31 to value 0
impl crate::Resettable for R31rs {}
