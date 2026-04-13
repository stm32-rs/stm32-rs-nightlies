///Register `FDCAN_TXBC` reader
pub type R = crate::R<FDCAN_TXBCrs>;
///Register `FDCAN_TXBC` writer
pub type W = crate::W<FDCAN_TXBCrs>;
/**Tx FIFO/queue mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TFQM {
    ///0: Tx FIFO operation
    B0x0 = 0,
    ///1: Tx queue operation.
    B0x1 = 1,
}
impl From<TFQM> for bool {
    #[inline(always)]
    fn from(variant: TFQM) -> Self {
        variant as u8 != 0
    }
}
///Field `TFQM` reader - Tx FIFO/queue mode
pub type TFQM_R = crate::BitReader<TFQM>;
impl TFQM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TFQM {
        match self.bits {
            false => TFQM::B0x0,
            true => TFQM::B0x1,
        }
    }
    ///Tx FIFO operation
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TFQM::B0x0
    }
    ///Tx queue operation.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TFQM::B0x1
    }
}
///Field `TFQM` writer - Tx FIFO/queue mode
pub type TFQM_W<'a, REG> = crate::BitWriter<'a, REG, TFQM>;
impl<'a, REG> TFQM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Tx FIFO operation
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TFQM::B0x0)
    }
    ///Tx queue operation.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TFQM::B0x1)
    }
}
impl R {
    ///Bit 24 - Tx FIFO/queue mode
    #[inline(always)]
    pub fn tfqm(&self) -> TFQM_R {
        TFQM_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TXBC")
            .field("tfqm", &self.tfqm())
            .finish()
    }
}
impl W {
    ///Bit 24 - Tx FIFO/queue mode
    #[inline(always)]
    pub fn tfqm(&mut self) -> TFQM_W<'_, FDCAN_TXBCrs> {
        TFQM_W::new(self, 24)
    }
}
/**FDCAN Tx buffer configuration register

You can [`read`](crate::Reg::read) this register and get [`fdcan_txbc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txbc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#FDCAN:FDCAN_TXBC)*/
pub struct FDCAN_TXBCrs;
impl crate::RegisterSpec for FDCAN_TXBCrs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_txbc::R`](R) reader structure
impl crate::Readable for FDCAN_TXBCrs {}
///`write(|w| ..)` method takes [`fdcan_txbc::W`](W) writer structure
impl crate::Writable for FDCAN_TXBCrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FDCAN_TXBC to value 0
impl crate::Resettable for FDCAN_TXBCrs {}
