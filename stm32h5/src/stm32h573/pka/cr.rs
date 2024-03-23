#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Field `EN` reader - PKA enable. When an illegal operation is selected while EN=1 OPERRF bit is set in PKA_SR. See PKA_CR.MODE bitfield for details. When EN=0 PKA RAM can still be accessed by the application."]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - PKA enable. When an illegal operation is selected while EN=1 OPERRF bit is set in PKA_SR. See PKA_CR.MODE bitfield for details. When EN=0 PKA RAM can still be accessed by the application."]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START` reader - start the operation Writing 1 to this bit starts the operation which is selected by MODE\\[5:0\\], using the operands and data already written to the PKA RAM. This bit is always read as 0. When an illegal operation is selected while START bit is set no operation is started, and OPERRF bit is set in PKA_SR. START is ignored if PKA is busy."]
pub type START_R = crate::BitReader;
#[doc = "Field `START` writer - start the operation Writing 1 to this bit starts the operation which is selected by MODE\\[5:0\\], using the operands and data already written to the PKA RAM. This bit is always read as 0. When an illegal operation is selected while START bit is set no operation is started, and OPERRF bit is set in PKA_SR. START is ignored if PKA is busy."]
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE` reader - PKA operation code When an operation not listed here is written by the application with EN bit set, OPERRF bit is set in PKA_SR register, and the write to MODE bitfield is ignored. When PKA is configured in limited mode (LMF = 1 in PKA_SR), writing a MODE different from 0x26 with EN bit to 1 triggers OPERRF bit to be set and write to MODE bit is ignored."]
pub type MODE_R = crate::FieldReader;
#[doc = "Field `MODE` writer - PKA operation code When an operation not listed here is written by the application with EN bit set, OPERRF bit is set in PKA_SR register, and the write to MODE bitfield is ignored. When PKA is configured in limited mode (LMF = 1 in PKA_SR), writing a MODE different from 0x26 with EN bit to 1 triggers OPERRF bit to be set and write to MODE bit is ignored."]
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PROCENDIE` reader - End of operation interrupt enable"]
pub type PROCENDIE_R = crate::BitReader;
#[doc = "Field `PROCENDIE` writer - End of operation interrupt enable"]
pub type PROCENDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMERRIE` reader - RAM error interrupt enable"]
pub type RAMERRIE_R = crate::BitReader;
#[doc = "Field `RAMERRIE` writer - RAM error interrupt enable"]
pub type RAMERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRERRIE` reader - Address error interrupt enable"]
pub type ADDRERRIE_R = crate::BitReader;
#[doc = "Field `ADDRERRIE` writer - Address error interrupt enable"]
pub type ADDRERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPERRIE` reader - Operation error interrupt enable"]
pub type OPERRIE_R = crate::BitReader;
#[doc = "Field `OPERRIE` writer - Operation error interrupt enable"]
pub type OPERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PKA enable. When an illegal operation is selected while EN=1 OPERRF bit is set in PKA_SR. See PKA_CR.MODE bitfield for details. When EN=0 PKA RAM can still be accessed by the application."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - start the operation Writing 1 to this bit starts the operation which is selected by MODE\\[5:0\\], using the operands and data already written to the PKA RAM. This bit is always read as 0. When an illegal operation is selected while START bit is set no operation is started, and OPERRF bit is set in PKA_SR. START is ignored if PKA is busy."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:13 - PKA operation code When an operation not listed here is written by the application with EN bit set, OPERRF bit is set in PKA_SR register, and the write to MODE bitfield is ignored. When PKA is configured in limited mode (LMF = 1 in PKA_SR), writing a MODE different from 0x26 with EN bit to 1 triggers OPERRF bit to be set and write to MODE bit is ignored."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 17 - End of operation interrupt enable"]
    #[inline(always)]
    pub fn procendie(&self) -> PROCENDIE_R {
        PROCENDIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - RAM error interrupt enable"]
    #[inline(always)]
    pub fn ramerrie(&self) -> RAMERRIE_R {
        RAMERRIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Address error interrupt enable"]
    #[inline(always)]
    pub fn addrerrie(&self) -> ADDRERRIE_R {
        ADDRERRIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Operation error interrupt enable"]
    #[inline(always)]
    pub fn operrie(&self) -> OPERRIE_R {
        OPERRIE_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PKA enable. When an illegal operation is selected while EN=1 OPERRF bit is set in PKA_SR. See PKA_CR.MODE bitfield for details. When EN=0 PKA RAM can still be accessed by the application."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CRrs> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - start the operation Writing 1 to this bit starts the operation which is selected by MODE\\[5:0\\], using the operands and data already written to the PKA RAM. This bit is always read as 0. When an illegal operation is selected while START bit is set no operation is started, and OPERRF bit is set in PKA_SR. START is ignored if PKA is busy."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<CRrs> {
        START_W::new(self, 1)
    }
    #[doc = "Bits 8:13 - PKA operation code When an operation not listed here is written by the application with EN bit set, OPERRF bit is set in PKA_SR register, and the write to MODE bitfield is ignored. When PKA is configured in limited mode (LMF = 1 in PKA_SR), writing a MODE different from 0x26 with EN bit to 1 triggers OPERRF bit to be set and write to MODE bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<CRrs> {
        MODE_W::new(self, 8)
    }
    #[doc = "Bit 17 - End of operation interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn procendie(&mut self) -> PROCENDIE_W<CRrs> {
        PROCENDIE_W::new(self, 17)
    }
    #[doc = "Bit 19 - RAM error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ramerrie(&mut self) -> RAMERRIE_W<CRrs> {
        RAMERRIE_W::new(self, 19)
    }
    #[doc = "Bit 20 - Address error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn addrerrie(&mut self) -> ADDRERRIE_W<CRrs> {
        ADDRERRIE_W::new(self, 20)
    }
    #[doc = "Bit 21 - Operation error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn operrie(&mut self) -> OPERRIE_W<CRrs> {
        OPERRIE_W::new(self, 21)
    }
}
#[doc = "PKA control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CRrs {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}
