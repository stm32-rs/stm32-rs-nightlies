///Register `R%s` reader
pub type R = crate::R<Rrs>;
///Register `R%s` writer
pub type W = crate::W<Rrs>;
///Field `PROCID` reader - Semaphore PROCID Written by software -When the semaphore is free and the LOCK is written to 1, and the LOCKID matches the AHB bus master ID, SEC and PRIV matches the AHB bus master definition, PROCID is set to the written data. - When the semaphore is unlocked, LOCK written to 0 and AHB bus master ID matched LOCKID, SEC and PRIV matches the AHB bus master protection, the PROCID is cleared to 0. - When the semaphore is unlocked, LOCK bit written to 0 and AHB bus master ID does not match LOCKID and/or SEC or PRIV do not match the AHB bus master definition, the PROCID is not affected. - Write when LOCK bit is already 1 (semaphore locked), the PROCID is not affected. - An authorized read returns the stored PROCID value.
pub type PROCID_R = crate::FieldReader;
///Field `PROCID` writer - Semaphore PROCID Written by software -When the semaphore is free and the LOCK is written to 1, and the LOCKID matches the AHB bus master ID, SEC and PRIV matches the AHB bus master definition, PROCID is set to the written data. - When the semaphore is unlocked, LOCK written to 0 and AHB bus master ID matched LOCKID, SEC and PRIV matches the AHB bus master protection, the PROCID is cleared to 0. - When the semaphore is unlocked, LOCK bit written to 0 and AHB bus master ID does not match LOCKID and/or SEC or PRIV do not match the AHB bus master definition, the PROCID is not affected. - Write when LOCK bit is already 1 (semaphore locked), the PROCID is not affected. - An authorized read returns the stored PROCID value.
pub type PROCID_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `LOCKID` reader - Semaphore LOCKID Written by software - When the semaphore is free and the LOCK bit is at the same time written to 1 and the LOCKID matches the AHB bus master ID, SEC and PRIV matches the AHB bus master protection. - When the semaphore is unlocked (LOCK written to 0 and AHB bus master ID matched LOCKID, SEC and PRIV matches the AHB bus master protection, the LOCKID is cleared to 0. - When the semaphore is unlocked (LOCK bit written to 0 or AHB bus master ID does not match LOCKID and/or SEC or PRIV do not match AHB bus master protection, the LOCKID is not affected. - Write when LOCK bit is already 1 (semaphore locked), the LOCKID is not affected. - An authorized read returns the stored LOCKID value.
pub type LOCKID_R = crate::FieldReader;
///Field `LOCKID` writer - Semaphore LOCKID Written by software - When the semaphore is free and the LOCK bit is at the same time written to 1 and the LOCKID matches the AHB bus master ID, SEC and PRIV matches the AHB bus master protection. - When the semaphore is unlocked (LOCK written to 0 and AHB bus master ID matched LOCKID, SEC and PRIV matches the AHB bus master protection, the LOCKID is cleared to 0. - When the semaphore is unlocked (LOCK bit written to 0 or AHB bus master ID does not match LOCKID and/or SEC or PRIV do not match AHB bus master protection, the LOCKID is not affected. - Write when LOCK bit is already 1 (semaphore locked), the LOCKID is not affected. - An authorized read returns the stored LOCKID value.
pub type LOCKID_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SEC` reader - Semaphore secure Written by software - When the semaphore is free and the LOCK bit is at the same time written to 1, and the LOCKID matches the AHB bus master ID, SEC and PRIV matches the AHB bus master definition. - When the semaphore is unlocked (LOCK written to 0 and AHB bus master ID matched LOCKID, SEC and PRIV matches the AHB bus master definition, the SEC is cleared to 0. - When the semaphore is unlocked (LOCK bit written to 0 and AHB bus master ID does not match LOCKID and/or SEC or PRIV do not match AHB bus master definition, the SEC is not affected. - Write when LOCK bit is already 1 (semaphore locked), the SEC is not affected. - An authorized read returns the stored SEC value.
pub type SEC_R = crate::BitReader;
///Field `SEC` writer - Semaphore secure Written by software - When the semaphore is free and the LOCK bit is at the same time written to 1, and the LOCKID matches the AHB bus master ID, SEC and PRIV matches the AHB bus master definition. - When the semaphore is unlocked (LOCK written to 0 and AHB bus master ID matched LOCKID, SEC and PRIV matches the AHB bus master definition, the SEC is cleared to 0. - When the semaphore is unlocked (LOCK bit written to 0 and AHB bus master ID does not match LOCKID and/or SEC or PRIV do not match AHB bus master definition, the SEC is not affected. - Write when LOCK bit is already 1 (semaphore locked), the SEC is not affected. - An authorized read returns the stored SEC value.
pub type SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIV` reader - Semaphore privilege Written by software - When the semaphore is free and the LOCK bit is at the same time written to 1, and the LOCKID matches the AHB bus master ID, SEC and PRIV matches the AHB bus master definition. - When the semaphore is unlocked (LOCK written to 0 and AHB bus master ID matched LOCKID, SEC and PRIV matches the AHB bus master definition, the PRIV is cleared to 0. - When the semaphore is unlocked (LOCK bit written to 0 and AHB bus master ID does not match LOCKID and/or SEC or PRIV do not match AHB bus master definition, the PRIV is not affected. - Write when LOCK bit is already 1 (semaphore locked), the PRIV is not affected. - An authorized read returns the stored PRIV value.
pub type PRIV_R = crate::BitReader;
///Field `PRIV` writer - Semaphore privilege Written by software - When the semaphore is free and the LOCK bit is at the same time written to 1, and the LOCKID matches the AHB bus master ID, SEC and PRIV matches the AHB bus master definition. - When the semaphore is unlocked (LOCK written to 0 and AHB bus master ID matched LOCKID, SEC and PRIV matches the AHB bus master definition, the PRIV is cleared to 0. - When the semaphore is unlocked (LOCK bit written to 0 and AHB bus master ID does not match LOCKID and/or SEC or PRIV do not match AHB bus master definition, the PRIV is not affected. - Write when LOCK bit is already 1 (semaphore locked), the PRIV is not affected. - An authorized read returns the stored PRIV value.
pub type PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LOCK` reader - Lock indication This bit can be written and read by software.
pub type LOCK_R = crate::BitReader;
///Field `LOCK` writer - Lock indication This bit can be written and read by software.
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - Semaphore PROCID Written by software -When the semaphore is free and the LOCK is written to 1, and the LOCKID matches the AHB bus master ID, SEC and PRIV matches the AHB bus master definition, PROCID is set to the written data. - When the semaphore is unlocked, LOCK written to 0 and AHB bus master ID matched LOCKID, SEC and PRIV matches the AHB bus master protection, the PROCID is cleared to 0. - When the semaphore is unlocked, LOCK bit written to 0 and AHB bus master ID does not match LOCKID and/or SEC or PRIV do not match the AHB bus master definition, the PROCID is not affected. - Write when LOCK bit is already 1 (semaphore locked), the PROCID is not affected. - An authorized read returns the stored PROCID value.
    #[inline(always)]
    pub fn procid(&self) -> PROCID_R {
        PROCID_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Semaphore LOCKID Written by software - When the semaphore is free and the LOCK bit is at the same time written to 1 and the LOCKID matches the AHB bus master ID, SEC and PRIV matches the AHB bus master protection. - When the semaphore is unlocked (LOCK written to 0 and AHB bus master ID matched LOCKID, SEC and PRIV matches the AHB bus master protection, the LOCKID is cleared to 0. - When the semaphore is unlocked (LOCK bit written to 0 or AHB bus master ID does not match LOCKID and/or SEC or PRIV do not match AHB bus master protection, the LOCKID is not affected. - Write when LOCK bit is already 1 (semaphore locked), the LOCKID is not affected. - An authorized read returns the stored LOCKID value.
    #[inline(always)]
    pub fn lockid(&self) -> LOCKID_R {
        LOCKID_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 12 - Semaphore secure Written by software - When the semaphore is free and the LOCK bit is at the same time written to 1, and the LOCKID matches the AHB bus master ID, SEC and PRIV matches the AHB bus master definition. - When the semaphore is unlocked (LOCK written to 0 and AHB bus master ID matched LOCKID, SEC and PRIV matches the AHB bus master definition, the SEC is cleared to 0. - When the semaphore is unlocked (LOCK bit written to 0 and AHB bus master ID does not match LOCKID and/or SEC or PRIV do not match AHB bus master definition, the SEC is not affected. - Write when LOCK bit is already 1 (semaphore locked), the SEC is not affected. - An authorized read returns the stored SEC value.
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Semaphore privilege Written by software - When the semaphore is free and the LOCK bit is at the same time written to 1, and the LOCKID matches the AHB bus master ID, SEC and PRIV matches the AHB bus master definition. - When the semaphore is unlocked (LOCK written to 0 and AHB bus master ID matched LOCKID, SEC and PRIV matches the AHB bus master definition, the PRIV is cleared to 0. - When the semaphore is unlocked (LOCK bit written to 0 and AHB bus master ID does not match LOCKID and/or SEC or PRIV do not match AHB bus master definition, the PRIV is not affected. - Write when LOCK bit is already 1 (semaphore locked), the PRIV is not affected. - An authorized read returns the stored PRIV value.
    #[inline(always)]
    pub fn priv_(&self) -> PRIV_R {
        PRIV_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 31 - Lock indication This bit can be written and read by software.
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("R")
            .field("procid", &self.procid())
            .field("lockid", &self.lockid())
            .field("sec", &self.sec())
            .field("priv_", &self.priv_())
            .field("lock", &self.lock())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Semaphore PROCID Written by software -When the semaphore is free and the LOCK is written to 1, and the LOCKID matches the AHB bus master ID, SEC and PRIV matches the AHB bus master definition, PROCID is set to the written data. - When the semaphore is unlocked, LOCK written to 0 and AHB bus master ID matched LOCKID, SEC and PRIV matches the AHB bus master protection, the PROCID is cleared to 0. - When the semaphore is unlocked, LOCK bit written to 0 and AHB bus master ID does not match LOCKID and/or SEC or PRIV do not match the AHB bus master definition, the PROCID is not affected. - Write when LOCK bit is already 1 (semaphore locked), the PROCID is not affected. - An authorized read returns the stored PROCID value.
    #[inline(always)]
    pub fn procid(&mut self) -> PROCID_W<'_, Rrs> {
        PROCID_W::new(self, 0)
    }
    ///Bits 8:11 - Semaphore LOCKID Written by software - When the semaphore is free and the LOCK bit is at the same time written to 1 and the LOCKID matches the AHB bus master ID, SEC and PRIV matches the AHB bus master protection. - When the semaphore is unlocked (LOCK written to 0 and AHB bus master ID matched LOCKID, SEC and PRIV matches the AHB bus master protection, the LOCKID is cleared to 0. - When the semaphore is unlocked (LOCK bit written to 0 or AHB bus master ID does not match LOCKID and/or SEC or PRIV do not match AHB bus master protection, the LOCKID is not affected. - Write when LOCK bit is already 1 (semaphore locked), the LOCKID is not affected. - An authorized read returns the stored LOCKID value.
    #[inline(always)]
    pub fn lockid(&mut self) -> LOCKID_W<'_, Rrs> {
        LOCKID_W::new(self, 8)
    }
    ///Bit 12 - Semaphore secure Written by software - When the semaphore is free and the LOCK bit is at the same time written to 1, and the LOCKID matches the AHB bus master ID, SEC and PRIV matches the AHB bus master definition. - When the semaphore is unlocked (LOCK written to 0 and AHB bus master ID matched LOCKID, SEC and PRIV matches the AHB bus master definition, the SEC is cleared to 0. - When the semaphore is unlocked (LOCK bit written to 0 and AHB bus master ID does not match LOCKID and/or SEC or PRIV do not match AHB bus master definition, the SEC is not affected. - Write when LOCK bit is already 1 (semaphore locked), the SEC is not affected. - An authorized read returns the stored SEC value.
    #[inline(always)]
    pub fn sec(&mut self) -> SEC_W<'_, Rrs> {
        SEC_W::new(self, 12)
    }
    ///Bit 13 - Semaphore privilege Written by software - When the semaphore is free and the LOCK bit is at the same time written to 1, and the LOCKID matches the AHB bus master ID, SEC and PRIV matches the AHB bus master definition. - When the semaphore is unlocked (LOCK written to 0 and AHB bus master ID matched LOCKID, SEC and PRIV matches the AHB bus master definition, the PRIV is cleared to 0. - When the semaphore is unlocked (LOCK bit written to 0 and AHB bus master ID does not match LOCKID and/or SEC or PRIV do not match AHB bus master definition, the PRIV is not affected. - Write when LOCK bit is already 1 (semaphore locked), the PRIV is not affected. - An authorized read returns the stored PRIV value.
    #[inline(always)]
    pub fn priv_(&mut self) -> PRIV_W<'_, Rrs> {
        PRIV_W::new(self, 13)
    }
    ///Bit 31 - Lock indication This bit can be written and read by software.
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<'_, Rrs> {
        LOCK_W::new(self, 31)
    }
}
/**HSEM register HSEM_R%s

You can [`read`](crate::Reg::read) this register and get [`r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#HSEM:R[0])*/
pub struct Rrs;
impl crate::RegisterSpec for Rrs {
    type Ux = u32;
}
///`read()` method returns [`r::R`](R) reader structure
impl crate::Readable for Rrs {}
///`write(|w| ..)` method takes [`r::W`](W) writer structure
impl crate::Writable for Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets R%s to value 0
impl crate::Resettable for Rrs {}
