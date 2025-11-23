///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `EN` reader - PKA enable. When an illegal operation is selected while EN=1 OPERRF bit is set in PKA_SR. See PKA_CR.MODE bitfield for details. Note: When EN=0 PKA RAM can still be accessed by the application.
pub type EN_R = crate::BitReader;
///Field `EN` writer - PKA enable. When an illegal operation is selected while EN=1 OPERRF bit is set in PKA_SR. See PKA_CR.MODE bitfield for details. Note: When EN=0 PKA RAM can still be accessed by the application.
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `START` reader - start the operation Writing 1 to this bit starts the operation which is selected by MODE\[5:0\], using the operands and data already written to the PKA RAM. This bit is always read as 0. When an illegal operation is selected while START bit is set no operation is started, and OPERRF bit is set in PKA_SR. Note: START is ignored if PKA is busy.
pub type START_R = crate::BitReader;
///Field `START` writer - start the operation Writing 1 to this bit starts the operation which is selected by MODE\[5:0\], using the operands and data already written to the PKA RAM. This bit is always read as 0. When an illegal operation is selected while START bit is set no operation is started, and OPERRF bit is set in PKA_SR. Note: START is ignored if PKA is busy.
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MODE` reader - PKA operation code When an operation not listed here is written by the application with EN bit set, OPERRF bit is set in PKA_SR register, and the write to MODE bitfield is ignored. When PKA is configured in limited mode (LMF = 1 in PKA_SR), writing a MODE different from 0x26 with EN bit to 1 triggers OPERRF bit to be set and write to MODE bit is ignored.
pub type MODE_R = crate::FieldReader;
///Field `MODE` writer - PKA operation code When an operation not listed here is written by the application with EN bit set, OPERRF bit is set in PKA_SR register, and the write to MODE bitfield is ignored. When PKA is configured in limited mode (LMF = 1 in PKA_SR), writing a MODE different from 0x26 with EN bit to 1 triggers OPERRF bit to be set and write to MODE bit is ignored.
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `PROCENDIE` reader - End of operation interrupt enable
pub type PROCENDIE_R = crate::BitReader;
///Field `PROCENDIE` writer - End of operation interrupt enable
pub type PROCENDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RAMERRIE` reader - RAM error interrupt enable
pub type RAMERRIE_R = crate::BitReader;
///Field `RAMERRIE` writer - RAM error interrupt enable
pub type RAMERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADDRERRIE` reader - Address error interrupt enable
pub type ADDRERRIE_R = crate::BitReader;
///Field `ADDRERRIE` writer - Address error interrupt enable
pub type ADDRERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPERRIE` reader - Operation error interrupt enable
pub type OPERRIE_R = crate::BitReader;
///Field `OPERRIE` writer - Operation error interrupt enable
pub type OPERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - PKA enable. When an illegal operation is selected while EN=1 OPERRF bit is set in PKA_SR. See PKA_CR.MODE bitfield for details. Note: When EN=0 PKA RAM can still be accessed by the application.
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - start the operation Writing 1 to this bit starts the operation which is selected by MODE\[5:0\], using the operands and data already written to the PKA RAM. This bit is always read as 0. When an illegal operation is selected while START bit is set no operation is started, and OPERRF bit is set in PKA_SR. Note: START is ignored if PKA is busy.
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 8:13 - PKA operation code When an operation not listed here is written by the application with EN bit set, OPERRF bit is set in PKA_SR register, and the write to MODE bitfield is ignored. When PKA is configured in limited mode (LMF = 1 in PKA_SR), writing a MODE different from 0x26 with EN bit to 1 triggers OPERRF bit to be set and write to MODE bit is ignored.
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    ///Bit 17 - End of operation interrupt enable
    #[inline(always)]
    pub fn procendie(&self) -> PROCENDIE_R {
        PROCENDIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 19 - RAM error interrupt enable
    #[inline(always)]
    pub fn ramerrie(&self) -> RAMERRIE_R {
        RAMERRIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Address error interrupt enable
    #[inline(always)]
    pub fn addrerrie(&self) -> ADDRERRIE_R {
        ADDRERRIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Operation error interrupt enable
    #[inline(always)]
    pub fn operrie(&self) -> OPERRIE_R {
        OPERRIE_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("en", &self.en())
            .field("start", &self.start())
            .field("mode", &self.mode())
            .field("procendie", &self.procendie())
            .field("ramerrie", &self.ramerrie())
            .field("addrerrie", &self.addrerrie())
            .field("operrie", &self.operrie())
            .finish()
    }
}
impl W {
    ///Bit 0 - PKA enable. When an illegal operation is selected while EN=1 OPERRF bit is set in PKA_SR. See PKA_CR.MODE bitfield for details. Note: When EN=0 PKA RAM can still be accessed by the application.
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<'_, CRrs> {
        EN_W::new(self, 0)
    }
    ///Bit 1 - start the operation Writing 1 to this bit starts the operation which is selected by MODE\[5:0\], using the operands and data already written to the PKA RAM. This bit is always read as 0. When an illegal operation is selected while START bit is set no operation is started, and OPERRF bit is set in PKA_SR. Note: START is ignored if PKA is busy.
    #[inline(always)]
    pub fn start(&mut self) -> START_W<'_, CRrs> {
        START_W::new(self, 1)
    }
    ///Bits 8:13 - PKA operation code When an operation not listed here is written by the application with EN bit set, OPERRF bit is set in PKA_SR register, and the write to MODE bitfield is ignored. When PKA is configured in limited mode (LMF = 1 in PKA_SR), writing a MODE different from 0x26 with EN bit to 1 triggers OPERRF bit to be set and write to MODE bit is ignored.
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<'_, CRrs> {
        MODE_W::new(self, 8)
    }
    ///Bit 17 - End of operation interrupt enable
    #[inline(always)]
    pub fn procendie(&mut self) -> PROCENDIE_W<'_, CRrs> {
        PROCENDIE_W::new(self, 17)
    }
    ///Bit 19 - RAM error interrupt enable
    #[inline(always)]
    pub fn ramerrie(&mut self) -> RAMERRIE_W<'_, CRrs> {
        RAMERRIE_W::new(self, 19)
    }
    ///Bit 20 - Address error interrupt enable
    #[inline(always)]
    pub fn addrerrie(&mut self) -> ADDRERRIE_W<'_, CRrs> {
        ADDRERRIE_W::new(self, 20)
    }
    ///Bit 21 - Operation error interrupt enable
    #[inline(always)]
    pub fn operrie(&mut self) -> OPERRIE_W<'_, CRrs> {
        OPERRIE_W::new(self, 21)
    }
}
/**PKA control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#PKA:CR)*/
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
